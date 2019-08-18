mod attribute;

pub use self::attribute::*;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Attributes {
    #[serde(rename = "attribute")]
    attributes: Vec<Attribute>,
}

impl Attributes {
    pub(crate) fn iter(&self) -> impl Iterator<Item = &Attribute> {
        self.attributes.iter()
    }
}
