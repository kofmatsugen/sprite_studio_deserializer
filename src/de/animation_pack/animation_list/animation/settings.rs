use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AnimationSetting {
    fps: u32,
    #[serde(rename = "frameCount")]
    count: u32,
    sort_mode: String,
}

impl AnimationSetting {
    pub fn fps(&self) -> u32 {
        self.fps
    }

    pub fn count(&self) -> u32 {
        self.count
    }

    pub fn sort_mode(&self) -> &str {
        self.sort_mode.as_ref()
    }
}
