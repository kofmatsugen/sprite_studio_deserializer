mod animation;

pub use self::animation::*;
use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct AnimationList {
    #[serde(rename = "anime")]
    animations: Vec<Animation>,
}

impl AnimationList {
    pub fn iter(&self) -> impl Iterator<Item = &Animation> {
        self.animations.iter()
    }
}
