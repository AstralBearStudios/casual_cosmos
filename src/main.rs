mod base;
mod config;
mod level;
mod loader;
mod platform;

use base::AppState;
use bevy::prelude::*;
// use bevy_lunex::prelude::*;
use level::LevelPlugin;
use loader::LoaderPlugin;
use platform::PlatformPlugin;

fn main() {
    App::new()
        .add_plugins((PlatformPlugin, LoaderPlugin, LevelPlugin))
        // FIXME: only use for testing levels
        .insert_state(AppState::Level)
        .run();
}
