use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub enum EffectNodeType {
    Root,
    #[serde(rename = "Emmiter")]
    Emitter,
    Particle,
}
