mod part;

pub use part::*;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PartAnimations {
    part_anime: Vec<PartAnime>,
}

impl PartAnimations {
    pub(crate) fn iter(&self) -> impl Iterator<Item = &PartAnime> {
        self.part_anime.iter()
    }
}
