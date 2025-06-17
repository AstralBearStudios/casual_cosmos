use bevy::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Asset, TypePath, Debug)]
pub struct Config {
    pub background: Srgba,
}

#[derive(Resource)]
pub struct ConfigHandle(pub Handle<Config>);

// #[derive(Deserialize, Debug)]
// #[serde(untagged)]
// pub enum Background {
//     Color(Srgba),
// }

// use bevy::prelude::{Asset, Color, Handle, Image, Resource, Srgba, TypePath};
// use serde::de::Error;
// use serde::de::{self, VariantAccess};
// use serde::{Deserialize, Deserializer, Serialize, de::Visitor};
// use std::fmt;
// use thiserror::Error;

// #[derive(Error, Debug)]
// enum BackgroundError {
//     #[error("invalid key {0}")]
//     WrongKey(String),
// }
//

// Adapated from paramagnetic at:
// https://users.rust-lang.org/t/custom-serde-deserialize-impl-for-enum/112285
// struct ColorVisitor;
// impl<'de> Visitor<'de> for ColorVisitor {
//     type Value = Color;
//
//     fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
//         // TODO: improve error messages
//         formatter.write_str("wrong key")
//     }
//
//     fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
//     where
//         A: de::MapAccess<'de>,
//     {
//         let mut red: Option<f32> = None;
//         let mut green: Option<f32> = None;
//         let mut blue: Option<f32> = None;
//         let mut alpha: Option<f32> = None;
//
//         let mut extra: Option<A::Error> = None;
//
//         while let Some(key) = map.next_key::<String>()? {
//             match key.as_str() {
//                 "red" => red = Some(map.next_value()?),
//                 "green" => green = Some(map.next_value()?),
//                 "blue" => blue = Some(map.next_value()?),
//                 "alpha" => alpha = Some(map.next_value()?),
//                 _ => {
//                     extra = Some(A::Error::custom(
//                         "invalid key. Must be 'red', 'green', or 'blue'",
//                     ))
//                 }
//             }
//         }
//
//         // TODO: make this cleaner.
//         // Probably could use a map here?
//         match (red, green, blue, alpha, extra) {
//             (Some(red), Some(green), Some(blue), None, None) => Ok(Color::srgb(red, green, blue)),
//             (Some(red), Some(green), Some(blue), Some(alpha), None) => {
//                 Ok(Color::srgba(red, green, blue, alpha))
//             }
//             (_, _, _, _, Some(error)) => Err(error),
//             // TODO: make sure to list all errors!
//             _ => Err(A::Error::custom("error with another key")),
//         }
//     }
// }
//
// struct BackgroundVisitor;
//
// impl<'de> Visitor<'de> for BackgroundVisitor {
//     type Value = Background;
//
//     fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
//         // TODO: improve error messages
//         formatter.write_str("wrong key")
//     }
//
//     fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
//     where
//         A: de::EnumAccess<'de>,
//     {
//         let (_, variant_data): (String, _) = data.variant()?;
//
//         if let Ok(color) =
//             variant_data.struct_variant(&["red", "green", "blue", "alpha"], ColorVisitor)
//         {
//             return Ok(Background::Color(color));
//         }
//
//         // TODO: need to fix error checking in bevy_common_assets.
//         // Any error will cause a panic!
//         return Err(A::Error::custom("NOT IMPLEMENTED"));
//     }
// }
//
// impl<'de> Deserialize<'de> for Background {
//     fn deserialize<D>(deserializer: D) -> Result<Background, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         deserializer.deserialize_enum("background", &[], BackgroundVisitor)
//     }
// }
