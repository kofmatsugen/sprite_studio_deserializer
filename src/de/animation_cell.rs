mod cells;

pub use cells::*;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnimationCells {
    name: String,
    image_path: String,
    pixel_size: Float2,
    cells: Cells,
}

impl AnimationCells {
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn image_path(&self) -> &str {
        self.image_path.as_ref()
    }

    pub fn pixel_size(&self) -> (f32, f32) {
        (self.pixel_size.x(), self.pixel_size.y())
    }

    pub fn cells(&self) -> impl Iterator<Item = &Cell> {
        self.cells.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn xml_animation_cell_deserialize_test() {
        use serde_xml_rs::*;

        let s = include_str!("../../../data/sprite_studio/houou.ssce");

        let _data: AnimationCells = from_reader(s.as_bytes()).unwrap();
    }
}
