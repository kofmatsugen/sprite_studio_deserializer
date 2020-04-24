use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
//#[serde(tag = "value")]
pub enum BehaviorAttribute {
    // Basic { name: String },
}
