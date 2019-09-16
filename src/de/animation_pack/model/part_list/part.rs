mod bounds;
mod inherit_rate;
mod inherit_type;
mod part_type;

pub use self::bounds::*;
pub use self::inherit_rate::*;
pub use self::inherit_type::*;
pub use self::part_type::*;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Part {
    name: String,
    #[serde(rename = "arrayIndex")]
    index: i32,
    #[serde(rename = "parentIndex")]
    parent: i32,
    #[serde(rename = "type")]
    part_type: PartType,
    #[serde(rename = "boundsType")]
    bounds: BoundsType,
    #[serde(rename = "inheritType")]
    inherit_type: InheritType,
    #[serde(rename = "refAnimePack")]
    refference_anime_pack: Option<String>,
    #[serde(rename = "refAnime")]
    refference_anime: Option<String>,
    #[serde(rename = "ineheritRates")]
    inherit_flags: InheritFlag,
}

impl Part {
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn index(&self) -> i32 {
        self.index
    }

    pub fn parent(&self) -> i32 {
        self.parent
    }

    pub fn part_type(&self) -> &PartType {
        &self.part_type
    }

    pub fn bounds(&self) -> &BoundsType {
        &self.bounds
    }

    pub fn refference_animation(&self) -> Option<(&str, &str)> {
        match (
            self.refference_anime_pack.as_ref(),
            self.refference_anime.as_ref(),
        ) {
            (Some(pack), Some(anim)) => Some((pack, anim)),
            _ => None,
        }
    }
}
