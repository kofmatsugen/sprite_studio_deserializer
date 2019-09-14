use serde::Deserialize;

#[derive(Deserialize, Clone, Copy, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InheritFlag {
    #[serde(rename = "ALPH")]
    alpha: u8,
    #[serde(rename = "FLPH")]
    flip_horizontal: u8,
    #[serde(rename = "FLPV")]
    flip_vertical: u8,
    #[serde(rename = "HIDE")]
    hide: u8,
}
