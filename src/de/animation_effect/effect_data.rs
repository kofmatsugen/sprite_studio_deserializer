mod node;

pub use node::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EffectData {
    fps: usize,
    is_lock_rand_seed: usize,
    node_list: Vec<EffectNode>,
}
