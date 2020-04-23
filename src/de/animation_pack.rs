mod animation_list;
mod cell_map_names;
mod model;

pub use self::animation_list::*;
pub use self::cell_map_names::CellMapNames;
pub use self::model::*;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AnimationPack {
    name: String,
    #[serde(rename = "animeList")]
    animations: AnimationList,
    #[serde(rename = "Model")]
    model: Model,
    #[serde(rename = "cellmapNames")]
    cell_map_names: CellMapNames,
}

impl AnimationPack {
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
    pub fn parts(&self) -> impl Iterator<Item = &Part> {
        self.model.parts()
    }

    pub fn animations(&self) -> impl Iterator<Item = &Animation> {
        self.animations.iter()
    }

    pub fn get_animation(&self, name: &str) -> Option<&Animation> {
        self.animations().find(|anim| anim.name() == name)
    }

    pub fn get_part_by_name(&self, name: &str) -> Option<&Part> {
        self.parts().find(|part| part.name() == name)
    }

    pub fn get_part_by_index(&self, index: i32) -> Option<&Part> {
        if index < 0 {
            return None;
        }
        self.parts().find(|part| part.index() == index)
    }

    pub fn cell_map_names(&self) -> impl Iterator<Item = &str> {
        self.cell_map_names.values()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn xml_animation_pack_deserialize_test() {
        use serde_xml_rs::*;

        let s = include_str!(
            "../../../data/sprite_studio/template/template/character_template_2head.ssae"
        );

        let _data: AnimationPack = from_reader(s.as_bytes()).unwrap();
    }
}
