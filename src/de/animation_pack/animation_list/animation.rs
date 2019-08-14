mod label;
mod parts;
mod settings;

pub use label::*;
pub use parts::*;
use serde::Deserialize;
pub use settings::*;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Animation {
    name: String,
    settings: AnimationSetting,
    labels: Vec<Labels>,
    part_animes: PartAnimations,
}

impl Animation {
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn setting(&self) -> &AnimationSetting {
        &self.settings
    }

    pub fn labels(&self) -> impl Iterator<Item = &Labels> {
        self.labels.iter()
    }

    pub fn part_animes(&self) -> impl Iterator<Item = &PartAnime> {
        self.part_animes.iter()
    }
}
