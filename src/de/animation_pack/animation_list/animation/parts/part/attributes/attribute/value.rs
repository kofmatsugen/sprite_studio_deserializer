use serde::de::*;
use std::fmt;

#[derive(Debug)]
pub enum ValueType {
    Simple(f32),
    Speed(f32),
    LoopNum(u32),
    MapId(u32),
    Name(String),
    StartLabel(String),
    EndLabel(String),
    StartOffset(i32),
    EndOffset(i32),
}

#[derive(Debug)]
pub struct Value {
    values: Vec<ValueType>,
}

impl Value {
    pub fn values(&self) -> impl Iterator<Item = &ValueType> {
        self.values.iter()
    }
}

impl<'de> Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(ValueVisitor)
    }
}

struct ValueVisitor;

impl<'de> Visitor<'de> for ValueVisitor {
    type Value = Value;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("map")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut vec = vec![];
        while let Some(k) = map.next_key::<String>()? {
            match k.as_ref() {
                "$value" => {
                    let v = map.next_value()?;
                    vec.push(ValueType::Simple(v))
                }
                "mapId" => {
                    let v = map.next_value()?;
                    vec.push(ValueType::MapId(v));
                }
                "name" => {
                    let v = map.next_value()?;
                    vec.push(ValueType::Name(v));
                }
                "loopNum" => {
                    let v = map.next_value()?;
                    vec.push(ValueType::LoopNum(v));
                }
                "startLabel" => {
                    let v = map.next_value()?;
                    vec.push(ValueType::StartLabel(v));
                }
                "endLabel" => {
                    let v = map.next_value()?;
                    vec.push(ValueType::EndLabel(v));
                }
                "startOffset" => {
                    let v = map.next_value()?;
                    vec.push(ValueType::StartOffset(v));
                }
                "endOffset" => {
                    let v = map.next_value()?;
                    vec.push(ValueType::EndOffset(v));
                }
                "speed" => {
                    let v = map.next_value()?;
                    vec.push(ValueType::Speed(v));
                }
                _ => Err(A::Error::custom(&format!("unsupported value: {}", k)))?,
            }
        }

        Ok(Value { values: vec })
    }
}
