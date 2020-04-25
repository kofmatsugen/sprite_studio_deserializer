mod behavior;
mod node_type;

pub use behavior::*;
pub use node_type::*;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EffectNode {
    node: Vec<EffectNodeElem>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EffectNodeElem {
    name: String,
    #[serde(rename = "type")]
    node_type: EffectNodeType,
    parent_index: i32,
    visible: u8,
    behavior: Option<EffectBehavior>,
}
