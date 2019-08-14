mod part_list;

pub use self::part_list::*;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[serde(rename = "partList")]
    parts: PartList,
}

impl Model {
    pub(crate) fn parts(&self) -> impl Iterator<Item = &Part> {
        self.parts.iter()
    }
}
