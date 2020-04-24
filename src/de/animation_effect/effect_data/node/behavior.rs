mod blend_type;
mod list;

pub use blend_type::*;
pub use list::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct EffectBehavior {
    #[serde(rename = "CellName")]
    cell_name: String,
    #[serde(rename = "CellMapName")]
    cell_map_name: String,
    #[serde(rename = "BlendType")]
    blend_type: BlendType,
}
