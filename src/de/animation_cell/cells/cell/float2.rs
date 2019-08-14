use serde::{de::*, Deserialize, Deserializer};
use std::fmt;

#[derive(Debug)]
pub struct Float2 {
    x: f32,
    y: f32,
}

impl Float2 {
    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }
}

impl<'de> Deserialize<'de> for Float2 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(Float2Visitor)
    }
}

struct Float2Visitor;

impl<'de> Visitor<'de> for Float2Visitor {
    type Value = Float2;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Float2 format is (x y)")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut x = None;
        let mut y = None;
        while let Some((k, v)) = map.next_entry::<String, String>()? {
            match k.as_ref() {
                "$value" => {
                    let pos: Vec<f32> = v
                        .split(char::is_whitespace)
                        .filter_map(|v| v.parse().ok())
                        .collect();
                    x = pos.get(0).cloned();
                    y = pos.get(1).cloned();
                }
                _ => Err(A::Error::custom(&format!("unsupported value: {}", k)))?,
            }
        }

        let x = x.ok_or(A::Error::custom(&format!("x value is not found")))?;
        let y = y.ok_or(A::Error::custom(&format!("y value is not found")))?;

        Ok(Float2 { x, y })
    }
}
