use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct AnimationProject {
    #[serde(rename = "cellmapNames")]
    cell_map_names: CellMapNames,
    #[serde(rename = "animepackNames")]
    anime_map_names: AnimeMapNames,
}

impl AnimationProject {
    pub(crate) fn cell_maps(&self) -> impl Iterator<Item = &str> {
        self.cell_map_names.value.iter().map(|name| name.as_ref())
    }

    pub(crate) fn anim_packs(&self) -> impl Iterator<Item = &str> {
        self.anime_map_names.value.iter().map(|name| name.as_ref())
    }
}

#[derive(Debug, Deserialize)]
struct CellMapNames {
    value: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct AnimeMapNames {
    value: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn xml_animation_project_deserialize_test() {
        use serde_xml_rs::*;

        let s = include_str!("../../../data/sprite_studio/splash/splash1024/splash1024.sspj");

        let _data: AnimationProject = from_reader(s.as_bytes()).unwrap();

        println!("{:?}", _data);
    }
}
