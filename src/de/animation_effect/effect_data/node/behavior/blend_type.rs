use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub enum BlendType {
    Add,
    Mix,
}
