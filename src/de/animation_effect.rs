mod effect_data;

pub use effect_data::*;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AnimationEffect {
    name: String,
    effect_data: EffectData,
}

impl AnimationEffect {
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn xml_animation_pack_deserialize_test() {
        use serde_xml_rs::*;

        let s = include_str!("../../../data/sprite_studio/effect_allcheck/EffectParticleTurnToDirectionEnabled_dirMin.ssee");

        let _data: AnimationEffect = from_reader(s.as_bytes()).unwrap();
    }
}
