// use bevy::asset::AssetPath;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use serde::de;
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, de::Visitor};
use std::fmt;

// TODO: replace with String or AssetPath.
// const CONFIG_PATH: str = "tests/background_color.toml";
#[derive(AssetCollection, Resource)]
pub struct ConfigHandle {
    // #[asset(key = "config")]
    #[asset(path = "tests/background.toml")]
    pub inner: Handle<Config>,

    // TEST
    #[asset(path = "bevy/rpg/chars/gabe/gabe-idle-run.png")]
    pub worker: Handle<Image>,
}

#[derive(Deserialize, Asset, TypePath, Debug)]
pub struct Config {
    // pub background: Srgba,
    pub background: Background,
}

// TODO: allow a regular string (without a table
#[derive(Debug, Deserialize)]
pub struct TempPath {
    pub filepath: String,
}

#[derive(Debug)]
// #[serde(untagged)]
pub enum Background {
    Color(Color),
    // TODO: use AssetPath instead.
    // Hard to use due to lifetimes...
    ImagePath(TempPath),
}

// Adapted from paramagnetic at:
// https://users.rust-lang.org/t/custom-serde-deserialize-impl-for-enum/112285
struct BackgroundVisitor;

impl<'de> Visitor<'de> for BackgroundVisitor {
    type Value = Background;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: improve error messages
        formatter.write_str("must be a table with 'red', 'green', and 'blue' values")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut red: Option<f32> = None;
        let mut green: Option<f32> = None;
        let mut blue: Option<f32> = None;
        let mut alpha: Option<f32> = None;
        let mut filepath: Option<String> = None;

        let mut extra: Option<A::Error> = None;

        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "red" => red = Some(map.next_value()?),
                "green" => green = Some(map.next_value()?),
                "blue" => blue = Some(map.next_value()?),
                "alpha" => alpha = Some(map.next_value()?),
                "filepath" => filepath = Some(map.next_value()?),
                _ => {
                    extra = Some(A::Error::custom(
                        "invalid key. Must be 'red', 'green', or 'blue'",
                    ))
                }
            }
        }

        if let Some(filepath) = filepath {
            return Ok(Background::ImagePath(TempPath { filepath }));
        }

        const COLOR_SIZE: f32 = 256.;

        // TODO: make this cleaner.
        // Probably could use a map here?
        match (red, green, blue, alpha, extra) {
            (Some(red), Some(green), Some(blue), None, None) => Ok(Background::Color(Color::srgb(
                red / COLOR_SIZE,
                green / COLOR_SIZE,
                blue / COLOR_SIZE,
            ))),
            (Some(red), Some(green), Some(blue), Some(alpha), None) => {
                Ok(Background::Color(Color::srgba(
                    red / COLOR_SIZE,
                    green / COLOR_SIZE,
                    blue / COLOR_SIZE,
                    alpha,
                )))
            }
            (_, _, _, _, Some(error)) => Err(error),
            // TODO: make sure to list all errors!
            _ => Err(A::Error::custom("error with another key")),
        }
    }
}

impl<'de> Deserialize<'de> for Background {
    fn deserialize<D>(deserializer: D) -> Result<Background, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(BackgroundVisitor)
    }
}
