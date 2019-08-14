mod attributes;

pub use attributes::*;
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PartAnime {
    #[serde(rename = "partName")]
    name: String,
    attributes: Attributes,
}

impl PartAnime {
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn attributes(&self) -> impl Iterator<Item = &Attribute> {
        self.attributes.iter()
    }

    // 指定したタグのキーリストを取得する
    pub fn get_keys(
        &self,
        tag: AttributeTag,
    ) -> impl Iterator<Item = impl Iterator<Item = &KeyValue>> {
        self.attributes
            .iter()
            .filter(move |attr| *attr.tag() == tag)
            .map(|attr| attr.keys())
    }

    pub fn get_key_values<I>(
        &self,
        tags: I,
    ) -> BTreeMap<AttributeTag, impl Iterator<Item = impl Iterator<Item = &KeyValue>>>
    where
        I: IntoIterator<Item = AttributeTag>,
    {
        let mut values_map = BTreeMap::new();
        for tag in tags.into_iter() {
            values_map.insert(tag, self.get_keys(tag));
        }

        values_map
    }
}
