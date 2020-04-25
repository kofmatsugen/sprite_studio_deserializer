use serde::{de::*, Deserialize};
use std::fmt;

#[derive(Deserialize, Debug)]
pub struct BehaviorAtrributeList {
    #[serde(default)]
    value: Vec<BehaviorAttribute>,
}

#[derive(Debug)]
pub enum BehaviorAttribute {
    Basic {
        priority: u32,
        maximum_particle: u32, // 同時に生存可能なパーティクル最大数
        at_create_time: u32,   // 1回で生成するパーティクル数
        interval: u32,         // 生成間隔
        lifetime: u32,         // エミッタの生存期間
        angle: f32,
        angle_variance: f32,
        speed: (f32, f32),    // 初速度
        lifespan: (f32, f32), // パーティクル生存期間
    },
    Delay {
        delay_time: u32, // 親発生時から発生までの時間
    },
    TransColorFade {
        // アルファ値100%で見ることのできる，そう時間対する時間
        // この期間の外側にいるときはフェードイン・アウトする
        display_range: (f32, f32),
    },
    Gravity {
        gravity: (f32, f32),
    },
    InfinitEmit {
        calc_generate: u32,
    }, // 無限にパーティクル生成
    InitVertexColor {
        color: ((f32, f32, f32, f32), (f32, f32, f32, f32)),
    },
    TransVertexColor {
        color: ((f32, f32, f32, f32), (f32, f32, f32, f32)),
    },
    InitPosition {
        offset_x: (f32, f32),
        offset_y: (f32, f32),
    },
    OverWriteSeed {
        seed: usize,
    },
    InitRotation {
        rotation: (f32, f32),     // 初期角度
        add_rotation: (f32, f32), // 毎Fの回転角度
    },
    TransRotation {
        rotation_factor: f32,   // 目標速度(倍率)
        end_life_time_per: u32, //  ライフタイムに対して何%のときに目標速度に達するか
    },
    InitSize {
        size_x: (f32, f32),
        size_y: (f32, f32),
        scale: (f32, f32),
    },
    TransSize {
        size_x: (f32, f32),
        size_y: (f32, f32),
        scale: (f32, f32),
    },
    AddTangentiala {
        acceleration: (f32, f32),
    },
    TransSpeed {
        speed: (f32, f32),
    },
}

struct InnerColor(String);

impl Into<(f32, f32, f32, f32)> for InnerColor {
    fn into(self) -> (f32, f32, f32, f32) {
        log::info!("color => {}", self.0);
        let alpha = u8::from_str_radix(&self.0[0..2], 16).unwrap() as f32 / 255.;
        let red = u8::from_str_radix(&self.0[2..4], 16).unwrap() as f32 / 255.;
        let green = u8::from_str_radix(&self.0[4..6], 16).unwrap() as f32 / 255.;
        let blue = u8::from_str_radix(&self.0[6..8], 16).unwrap() as f32 / 255.;

        (red, green, blue, alpha)
    }
}

impl<'de> Deserialize<'de> for BehaviorAttribute {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(AttributeVisitor)
    }
}

// builder regex
// (\w+): (\w+),
// $1: Option<$2>,
#[derive(Default)]
struct BehaviorBuilder {
    name: Option<String>,

    priority: Option<u32>,
    maximum_particle: Option<u32>,
    at_create_time: Option<u32>,
    interval: Option<u32>,
    lifetime: Option<u32>,
    angle: Option<f32>,
    angle_variance: Option<f32>,
    speed: Option<(f32, f32)>,    // 初速度
    lifespan: Option<(f32, f32)>, // パーティクル生存期間

    delay_time: Option<u32>,

    display_range: Option<(f32, f32)>,

    gravity: Option<(f32, f32)>,

    calc_generate: Option<u32>,

    color: Option<((f32, f32, f32, f32), (f32, f32, f32, f32))>,

    offset_x: Option<(f32, f32)>,
    offset_y: Option<(f32, f32)>,

    seed: Option<usize>,

    rotation: Option<(f32, f32)>,     // 初期角度
    add_rotation: Option<(f32, f32)>, // 毎Fの回転角度

    rotation_factor: Option<f32>,   // 目標速度(倍率)
    end_life_time_per: Option<u32>, //  ライフタイムに対して何%のときに目標速度に達するか

    size_x: Option<(f32, f32)>,
    size_y: Option<(f32, f32)>,
    scale: Option<(f32, f32)>,

    acceleration: Option<(f32, f32)>,
}

#[derive(Deserialize, Debug)]
struct RangeValue<T> {
    value: T,
    subvalue: T,
}

struct AttributeVisitor;
impl<'de> Visitor<'de> for AttributeVisitor {
    type Value = BehaviorAttribute;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("map")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut builder = BehaviorBuilder::default();
        while let Some(k) = map.next_key::<String>()? {
            log::info!("key_name: {}", k);
            match k.as_ref() {
                "name" => {
                    let name = map.next_value()?;
                    if let Some(current) = builder.name.as_ref() {
                        log::warn!("conflict name: {} <=> {}", current, name);
                    }
                    builder.name = Some(name);
                }
                "priority" => {
                    let priority = map.next_value()?;
                    builder.priority = Some(priority);
                }
                "maximumParticle" => {
                    let maximum_particle = map.next_value()?;
                    builder.maximum_particle = Some(maximum_particle);
                }
                "attimeCreate" => {
                    let at_create_time = map.next_value()?;
                    builder.at_create_time = Some(at_create_time);
                }
                "interval" => {
                    let interval = map.next_value()?;
                    builder.interval = Some(interval);
                }
                "lifetime" => {
                    let lifetime = map.next_value()?;
                    builder.lifetime = Some(lifetime);
                }
                "angle" => {
                    let angle = map.next_value()?;
                    builder.angle = Some(angle);
                }
                "angleVariance" => {
                    let angle_variance = map.next_value()?;
                    builder.angle_variance = Some(angle_variance);
                }
                "speed" | "Speed" => {
                    let range: RangeValue<f32> = map.next_value()?;
                    builder.speed = Some((range.value, range.subvalue));
                }
                "lifespan" => {
                    let range: RangeValue<f32> = map.next_value()?;
                    builder.lifespan = Some((range.value, range.subvalue));
                }
                "DelayTime" => {
                    let delay_time = map.next_value()?;
                    builder.delay_time = Some(delay_time);
                }
                "disprange" => {
                    let range: RangeValue<f32> = map.next_value()?;
                    builder.display_range = Some((range.value, range.subvalue));
                }
                "Gravity" => {
                    let p_str = map.next_value::<String>()?;
                    let pos: Vec<f32> = p_str
                        .split(char::is_whitespace)
                        .filter_map(|v| v.parse().ok())
                        .collect();

                    builder.gravity = Some((
                        *pos.get(0)
                            .ok_or(A::Error::custom(&format!("gravity x value is not found")))?,
                        *pos.get(1)
                            .ok_or(A::Error::custom(&format!("gravity x value is not found")))?,
                    ));
                }
                "calcGen" => {
                    let calc_generate = map.next_value()?;
                    builder.calc_generate = Some(calc_generate);
                }
                "Color" => {
                    let color: RangeValue<String> = map.next_value()?;
                    let color = (
                        InnerColor(color.value).into(),
                        InnerColor(color.subvalue).into(),
                    );
                    builder.color = Some(color);
                }
                "OffsetX" => {
                    let offset_x: RangeValue<f32> = map.next_value()?;
                    builder.offset_x = Some((offset_x.value, offset_x.subvalue));
                }
                "OffsetY" => {
                    let offset_y: RangeValue<f32> = map.next_value()?;
                    builder.offset_y = Some((offset_y.value, offset_y.subvalue));
                }
                "Seed" => {
                    let seed = map.next_value()?;
                    builder.seed = Some(seed);
                }
                "Rotation" => {
                    let rotation: RangeValue<f32> = map.next_value()?;
                    builder.rotation = Some((rotation.value, rotation.subvalue));
                }
                "RotationAdd" => {
                    let add_rotation: RangeValue<f32> = map.next_value()?;
                    builder.add_rotation = Some((add_rotation.value, add_rotation.subvalue));
                }
                "RotationFactor" => {
                    let rotation_factor = map.next_value()?;
                    builder.rotation_factor = Some(rotation_factor);
                }
                "EndLifeTimePer" => {
                    let end_life_time_per = map.next_value()?;
                    builder.end_life_time_per = Some(end_life_time_per);
                }
                "SizeX" => {
                    let size_x: RangeValue<f32> = map.next_value()?;
                    builder.size_x = Some((size_x.value, size_x.subvalue));
                }
                "SizeY" => {
                    let size_y: RangeValue<f32> = map.next_value()?;
                    builder.size_y = Some((size_y.value, size_y.subvalue));
                }
                "ScaleFactor" => {
                    let scale: RangeValue<f32> = map.next_value()?;
                    builder.scale = Some((scale.value, scale.subvalue));
                }
                "Acceleration" => {
                    let acceleration: RangeValue<f32> = map.next_value()?;
                    builder.acceleration = Some((acceleration.value, acceleration.subvalue));
                }
                _ => Err(A::Error::custom(&format!("unsupported value: {}", k)))?,
            }
        }

        let name = builder.name.take();

        // builder regex
        // (\w+): \w+,
        // $1: builder.$1.ok_or(A::Error::custom("not set value $1"))?,

        let attr = match name.as_ref().map(String::as_str) {
            Some("Basic") => BehaviorAttribute::Basic {
                priority: builder
                    .priority
                    .ok_or(A::Error::custom("not set value priority"))?,
                maximum_particle: builder
                    .maximum_particle
                    .ok_or(A::Error::custom("not set value maximum_particle"))?,
                at_create_time: builder
                    .at_create_time
                    .ok_or(A::Error::custom("not set value at_create_time"))?,
                interval: builder
                    .interval
                    .ok_or(A::Error::custom("not set value interval"))?,
                lifetime: builder
                    .lifetime
                    .ok_or(A::Error::custom("not set value lifetime"))?,
                angle: builder
                    .angle
                    .ok_or(A::Error::custom("not set value angle"))?,
                angle_variance: builder
                    .angle_variance
                    .ok_or(A::Error::custom("not set value angle_variance"))?,
                speed: builder
                    .speed
                    .ok_or(A::Error::custom("not set value speed"))?, // 初速度
                lifespan: builder
                    .lifespan
                    .ok_or(A::Error::custom("not set value lifespan"))?, // パーティクル生存期間
            },
            Some("Delay") => BehaviorAttribute::Delay {
                delay_time: builder
                    .delay_time
                    .ok_or(A::Error::custom("not set value delay_time"))?,
            },
            Some("trans_colorfade") => BehaviorAttribute::TransColorFade {
                display_range: builder
                    .display_range
                    .ok_or(A::Error::custom("not set value display_range"))?,
            },
            Some("Gravity") => BehaviorAttribute::Gravity {
                gravity: builder
                    .gravity
                    .ok_or(A::Error::custom("not set value gravity"))?,
            },
            Some("InfiniteEmit") => BehaviorAttribute::InfinitEmit {
                calc_generate: builder
                    .calc_generate
                    .ok_or(A::Error::custom("not set value calc_generate"))?,
            },
            Some("init_vertexcolor") => BehaviorAttribute::InitVertexColor {
                color: builder
                    .color
                    .ok_or(A::Error::custom("not set value color"))?,
            },
            Some("init_position") => BehaviorAttribute::InitPosition {
                offset_x: builder
                    .offset_x
                    .ok_or(A::Error::custom("not set value offset_x"))?,
                offset_y: builder
                    .offset_y
                    .ok_or(A::Error::custom("not set value offset_y"))?,
            },
            Some("OverWriteSeed") => BehaviorAttribute::OverWriteSeed {
                seed: builder.seed.ok_or(A::Error::custom("not set value seed"))?,
            },
            Some("init_rotation") => BehaviorAttribute::InitRotation {
                rotation: builder
                    .rotation
                    .ok_or(A::Error::custom("not set value rotation"))?, // 初期角度
                add_rotation: builder
                    .add_rotation
                    .ok_or(A::Error::custom("not set value add_rotation"))?, // 毎Fの回転角度
            },
            Some("trans_rotation") => BehaviorAttribute::TransRotation {
                rotation_factor: builder
                    .rotation_factor
                    .ok_or(A::Error::custom("not set value rotation_factor"))?, // 目標速度(倍率)
                end_life_time_per: builder
                    .end_life_time_per
                    .ok_or(A::Error::custom("not set value end_life_time_per"))?, //  ライフタイムに対して何%のときに目標速度に達するか
            },
            Some("init_size") => BehaviorAttribute::InitSize {
                size_x: builder
                    .size_x
                    .ok_or(A::Error::custom("not set value size_x"))?,
                size_y: builder
                    .size_y
                    .ok_or(A::Error::custom("not set value size_y"))?,
                scale: builder
                    .scale
                    .ok_or(A::Error::custom("not set value scale"))?,
            },
            Some("trans_size") => BehaviorAttribute::TransSize {
                size_x: builder
                    .size_x
                    .ok_or(A::Error::custom("not set value size_x"))?,
                size_y: builder
                    .size_y
                    .ok_or(A::Error::custom("not set value size_y"))?,
                scale: builder
                    .scale
                    .ok_or(A::Error::custom("not set value scale"))?,
            },
            Some("add_tangentiala") => BehaviorAttribute::AddTangentiala {
                acceleration: builder
                    .acceleration
                    .ok_or(A::Error::custom("not set value acceleration"))?,
            },
            Some("trans_vertexcolor") => BehaviorAttribute::TransVertexColor {
                color: builder
                    .color
                    .ok_or(A::Error::custom("not set value color"))?,
            },
            Some("trans_speed") => BehaviorAttribute::TransSpeed {
                speed: builder
                    .speed
                    .ok_or(A::Error::custom("not set value speed"))?,
            },
            Some(name) => Err(A::Error::custom(&format!(
                "unsupported attribute name: {}",
                name
            )))?,
            None => Err(A::Error::custom(&format!("attribute name is none")))?,
        };
        Ok(attr)
    }
}
