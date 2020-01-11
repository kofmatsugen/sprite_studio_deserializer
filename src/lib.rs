mod de;

pub use de::*;

use failure::Fail;
use serde_xml_rs::*;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

#[derive(Debug, Fail)]
enum SerdeXmlError {
    #[fail(display = "{}", err)]
    Error { err: serde_xml_rs::ErrorKind },
}

#[derive(Default, Debug)]
pub struct SpriteStudioData {
    cell_maps: Vec<AnimationCells>,
    packs: Vec<AnimationPack>,
}

impl SpriteStudioData {
    pub fn cell_maps(&self) -> impl Iterator<Item = &AnimationCells> {
        self.cell_maps.iter()
    }

    pub fn packs(&self) -> impl Iterator<Item = &AnimationPack> {
        self.packs.iter()
    }

    pub fn cell(&self, map_id: usize, part_name: &str) -> Option<&Cell> {
        self.cell_maps
            .get(map_id)
            .and_then(|cell_map| cell_map.cells().find(|cell| cell.name() == part_name))
    }

    pub fn cell_index(&self, map_id: usize, part_name: &str) -> Option<usize> {
        self.cell_maps.get(map_id).and_then(|cell_map| {
            for (i, cell) in cell_map.cells().enumerate() {
                if cell.name() == part_name {
                    return Some(i);
                }
            }
            None
        })
    }
}

pub fn load_project<P: AsRef<Path>>(path: P) -> Result<SpriteStudioData, failure::Error> {
    let path = path.as_ref();
    let mut data = SpriteStudioData::default();

    let file = File::open(path)?;
    let buf_reader = BufReader::new(file);

    let project_data: AnimationProject = from_reader(buf_reader)
        .map_err(|serde_xml_rs::Error(err, _)| SerdeXmlError::Error { err })?;
    if let Some(parent) = path.parent() {
        for cell in project_data.cell_maps() {
            let mut path = PathBuf::new();
            path.push(parent);
            path.push(cell);
            let file = File::open(path)?;
            let buf_reader = BufReader::new(file);
            let cell_data: AnimationCells = from_reader(buf_reader)
                .map_err(|serde_xml_rs::Error(err, _)| SerdeXmlError::Error { err })?;
            data.cell_maps.push(cell_data);
        }
        for map in project_data.anim_packs() {
            let mut path = PathBuf::new();
            path.push(parent);
            path.push(map);
            let file = File::open(path)?;
            let buf_reader = BufReader::new(file);
            let pack_data: AnimationPack = from_reader(buf_reader)
                .map_err(|serde_xml_rs::Error(err, _)| SerdeXmlError::Error { err })?;
            data.packs.push(pack_data);
        }
    }

    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn load_test() {
        load_project("./data/houou.sspj").unwrap();
    }
}
