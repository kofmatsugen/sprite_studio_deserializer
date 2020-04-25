mod node;

pub use node::*;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EffectData {
    fps: usize,
    lock_rand_seed: usize,
    is_lock_rand_seed: usize,
    node_list: Vec<EffectNode>,
}
