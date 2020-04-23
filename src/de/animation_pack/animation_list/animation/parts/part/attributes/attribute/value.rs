use serde::de::*;
use serde::Deserialize;
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
    Integer(i32),
    Point(f32, f32),
    Rect(f32, f32, f32, f32),
    Text(String),
    Target(String),
    BlendType(String),
    Color(f32, f32, f32, f32),
    Infinity(bool),
    Reverse(bool),
    PingPong(bool),
    Indipendent(bool),
    VertexLT(f32, f32),
    VertexRT(f32, f32),
    VertexLB(f32, f32),
    VertexRB(f32, f32),
    DeformSize(usize), //Deform により移動する点
    Deform(Vec<(usize, f32, f32)>),
}

#[derive(Debug, Clone, Deserialize)]
struct InnerColor {
    pub rgba: String,
    pub rate: f32,
}

impl Into<(f32, f32, f32, f32)> for InnerColor {
    fn into(self) -> (f32, f32, f32, f32) {
        let alpha = u8::from_str_radix(&self.rgba[0..2], 16).unwrap() as f32 / 255.;
        let red = u8::from_str_radix(&self.rgba[2..4], 16).unwrap() as f32 / 255.;
        let green = u8::from_str_radix(&self.rgba[4..6], 16).unwrap() as f32 / 255.;
        let blue = u8::from_str_radix(&self.rgba[6..8], 16).unwrap() as f32 / 255.;

        (
            red * self.rate + 1. - self.rate,
            green * self.rate + 1. - self.rate,
            blue * self.rate + 1. - self.rate,
            alpha * self.rate + 1. - self.rate,
        )
    }
}
#[derive(Debug)]
pub(crate) struct Value {
    values: Vec<ValueType>,
}

impl Value {
    pub(crate) fn values(&self) -> impl Iterator<Item = &ValueType> {
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
                "integer" => {
                    let v = map.next_value()?;
                    vec.push(ValueType::Integer(v));
                }
                "string" => {
                    let v = map.next_value()?;
                    vec.push(ValueType::Text(v));
                }
                "point" => {
                    let p_str = map.next_value::<String>()?;
                    let pos: Vec<f32> = p_str
                        .split(char::is_whitespace)
                        .filter_map(|v| v.parse().ok())
                        .collect();
                    let x = pos.get(0).cloned();
                    let y = pos.get(1).cloned();
                    vec.push(ValueType::Point(
                        x.ok_or(A::Error::custom(&format!("x value is not found")))?,
                        y.ok_or(A::Error::custom(&format!("y value is not found")))?,
                    ));
                }
                "rect" => {
                    let p_str = map.next_value::<String>()?;
                    let pos: Vec<f32> = p_str
                        .split(char::is_whitespace)
                        .filter_map(|v| v.parse().ok())
                        .collect();
                    let x = pos.get(0).cloned();
                    let y = pos.get(1).cloned();
                    let w = pos.get(2).cloned();
                    let h = pos.get(3).cloned();
                    vec.push(ValueType::Rect(
                        x.ok_or(A::Error::custom(&format!("x value is not found")))?,
                        y.ok_or(A::Error::custom(&format!("y value is not found")))?,
                        w.ok_or(A::Error::custom(&format!("w value is not found")))?,
                        h.ok_or(A::Error::custom(&format!("h value is not found")))?,
                    ));
                }
                "color" => {
                    let inner = map.next_value::<InnerColor>()?;
                    let (r, g, b, a) = inner.into();
                    vec.push(ValueType::Color(r, g, b, a));
                }
                "target" => {
                    let v = map.next_value()?;
                    vec.push(ValueType::Target(v));
                }
                "blendType" => {
                    let v = map.next_value()?;
                    vec.push(ValueType::BlendType(v));
                }
                "infinity" => {
                    let v = map.next_value::<u8>()?;
                    let v = v == 1;
                    vec.push(ValueType::Infinity(v));
                }
                "reverse" => {
                    let v = map.next_value::<u8>()?;
                    let v = v == 1;
                    vec.push(ValueType::Reverse(v));
                }
                "pingpong" => {
                    let v = map.next_value::<u8>()?;
                    let v = v == 1;
                    vec.push(ValueType::PingPong(v));
                }
                "independent" => {
                    let v = map.next_value::<u8>()?;
                    let v = v == 1;
                    vec.push(ValueType::Indipendent(v));
                }
                "LT" => {
                    let p_str = map.next_value::<String>()?;
                    let pos: Vec<f32> = p_str
                        .split(char::is_whitespace)
                        .filter_map(|v| v.parse().ok())
                        .collect();
                    let x = pos.get(0).cloned();
                    let y = pos.get(1).cloned();
                    vec.push(ValueType::VertexLT(
                        x.ok_or(A::Error::custom(&format!("LT x value is not found")))?,
                        y.ok_or(A::Error::custom(&format!("LT x value is not found")))?,
                    ));
                }
                "RT" => {
                    let p_str = map.next_value::<String>()?;
                    let pos: Vec<f32> = p_str
                        .split(char::is_whitespace)
                        .filter_map(|v| v.parse().ok())
                        .collect();
                    let x = pos.get(0).cloned();
                    let y = pos.get(1).cloned();
                    vec.push(ValueType::VertexRT(
                        x.ok_or(A::Error::custom(&format!("RT x value is not found")))?,
                        y.ok_or(A::Error::custom(&format!("RT x value is not found")))?,
                    ));
                }
                "LB" => {
                    let p_str = map.next_value::<String>()?;
                    let pos: Vec<f32> = p_str
                        .split(char::is_whitespace)
                        .filter_map(|v| v.parse().ok())
                        .collect();
                    let x = pos.get(0).cloned();
                    let y = pos.get(1).cloned();
                    vec.push(ValueType::VertexLB(
                        x.ok_or(A::Error::custom(&format!("LB x value is not found")))?,
                        y.ok_or(A::Error::custom(&format!("LB x value is not found")))?,
                    ));
                }
                "RB" => {
                    let p_str = map.next_value::<String>()?;
                    let pos: Vec<f32> = p_str
                        .split(char::is_whitespace)
                        .filter_map(|v| v.parse().ok())
                        .collect();
                    let x = pos.get(0).cloned();
                    let y = pos.get(1).cloned();
                    vec.push(ValueType::VertexRB(
                        x.ok_or(A::Error::custom(&format!("RB x value is not found")))?,
                        y.ok_or(A::Error::custom(&format!("RB x value is not found")))?,
                    ));
                }
                "vsize" => {
                    let size = map.next_value::<String>()?;
                    let size = size.parse().map_err(|e| {
                        A::Error::custom(&format!("deform size fail parse: {}, {:?}", size, e))
                    })?;

                    vec.push(ValueType::DeformSize(size));
                }
                "vchg" => {
                    let p_str = map.next_value::<String>()?;
                    let pos = p_str.split(char::is_whitespace).skip(1).collect::<Vec<_>>();

                    let mut deforms = vec![];
                    let len = pos.len() / 3;

                    for i in 0..len {
                        match &pos[i * 3..i * 3 + 3] {
                            [index, x, y] => {
                                let index = index.parse().map_err(|e| {
                                    A::Error::custom(&format!(
                                        "deform index fail parse: {}, {:?}",
                                        index, e
                                    ))
                                })?;
                                let x = x.parse().map_err(|e| {
                                    A::Error::custom(&format!(
                                        "deform x fail parse: {}, {:?}",
                                        x, e
                                    ))
                                })?;
                                let y = y.parse().map_err(|e| {
                                    A::Error::custom(&format!(
                                        "deform y fail parse: {}, {:?}",
                                        y, e
                                    ))
                                })?;
                                deforms.push((index, x, y));
                            }
                            _ => {
                                Err(A::Error::custom(&format!(
                                    "deform value is not multiples of 3"
                                )))?;
                            }
                        }
                    }
                    vec.push(ValueType::Deform(deforms));
                }

                _ => Err(A::Error::custom(&format!("unsupported value: {}", k)))?,
            }
        }

        Ok(Value { values: vec })
    }
}
