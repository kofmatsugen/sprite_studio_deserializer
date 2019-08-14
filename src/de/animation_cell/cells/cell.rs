mod float2;

pub use float2::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cell {
    name: String,
    pos: Float2,
    pivot: Float2,
    size: Float2,
    rotated: f32,
}

impl Cell {
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn position(&self) -> (f32, f32) {
        (self.pos.x(), self.pos.y())
    }

    pub fn pivot(&self) -> (f32, f32) {
        (self.pivot.x(), self.pivot.y())
    }

    pub fn size(&self) -> (f32, f32) {
        (self.size.x(), self.size.y())
    }

    pub fn rotated(&self) -> f32 {
        self.rotated
    }
}
