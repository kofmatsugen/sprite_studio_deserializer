mod bounds;
mod part_type;

pub use self::bounds::*;
pub use self::part_type::*;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Part {
    name: String,
    #[serde(rename = "arrayIndex")]
    index: i32,
    #[serde(rename = "parentIndex")]
    parent: i32,
    #[serde(rename = "type")]
    part_type: PartType,
    #[serde(rename = "boundsType")]
    bounds: BoundsType,
}

impl Part {
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn index(&self) -> i32 {
        self.index
    }

    pub fn parent(&self) -> i32 {
        self.parent
    }

    pub fn part_type(&self) -> &PartType {
        &self.part_type
    }

    pub fn bounds(&self) -> &BoundsType {
        &self.bounds
    }
}
