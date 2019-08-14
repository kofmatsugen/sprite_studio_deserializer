use serde::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum BoundsType {
    None,
    Quad,
    Aabb,
    Circle,
    #[serde(rename = "circle_smin")]
    CircleMin,
    #[serde(rename = "circle_smax")]
    CircleMax,
}
