mod part;

pub use part::*;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PartList {
    value: Vec<Part>,
}

impl PartList {
    pub(crate) fn iter(&self) -> impl Iterator<Item = &Part> {
        self.value.iter()
    }
}
