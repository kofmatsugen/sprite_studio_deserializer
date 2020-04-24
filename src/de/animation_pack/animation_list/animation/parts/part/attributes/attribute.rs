mod value;

use serde::Deserialize;
pub use value::*;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Attribute {
    tag: AttributeTag,
    key: Vec<KeyValue>,
}

impl Attribute {
    pub fn tag(&self) -> &AttributeTag {
        &self.tag
    }

    pub fn keys(&self) -> impl Iterator<Item = &KeyValue> {
        self.key.iter()
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct KeyValue {
    time: u32,
    #[serde(default)]
    ip_type: Interpolation,
    value: Value,
}

impl KeyValue {
    pub fn time(&self) -> u32 {
        self.time
    }

    pub fn interpolation(&self) -> Interpolation {
        self.ip_type
    }

    pub fn values(&self) -> impl Iterator<Item = &ValueType> {
        self.value.values()
    }
}

#[derive(Deserialize, Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
#[serde(rename_all = "UPPERCASE")]
pub enum AttributeTag {
    Cell,
    Posx,
    Posy,
    Posz,
    Rotx,
    Roty,
    Rotz,
    Sclx,
    Scly,
    #[serde(rename = "ALPH")]
    Alpha,
    Prio,
    #[serde(rename = "FLPH")]
    Fliph,
    #[serde(rename = "FLPV")]
    Flipv,
    Hide,
    #[serde(rename = "PCOL")]
    Color,
    #[serde(rename = "VERT")]
    Vertex,
    #[serde(rename = "PVTX")]
    Pivotx,
    #[serde(rename = "PVTY")]
    Pivoty,
    #[serde(rename = "ANCX")]
    Anchorx,
    #[serde(rename = "ANCY")]
    Anchory,
    #[serde(rename = "SIZX")]
    Sizex,
    #[serde(rename = "SIZY")]
    Sizey,
    #[serde(rename = "IFLH")]
    Imgfliph,
    #[serde(rename = "IFLV")]
    Imgflipv,
    Uvtx,
    Uvty,
    Uvrz,
    Uvsx,
    Uvsy,
    Boundr,
    User,
    #[serde(rename = "IPRM")]
    Instance,
    #[serde(rename = "DEFM")]
    Deform,
    #[serde(rename = "EFCT")]
    Effect,
}

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum Interpolation {
    Linear,
    Hermite,
    Bezier,
    Acceleration,
    Deceleration,
    #[serde(other)]
    Step,
}

impl Default for Interpolation {
    fn default() -> Self {
        Interpolation::Step
    }
}
