use serde::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum PartType {
    Null,
    Normal,
    Text,
    Instance,
}
