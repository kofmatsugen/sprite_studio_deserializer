use serde::Deserialize;

#[derive(Deserialize, Clone, Copy, Debug)]
#[serde(rename_all = "lowercase")]
pub enum InheritType {
    #[serde(rename = "self")]
    NoInherit, // 継承しない
    Parent, // 親の値を継承する
}
