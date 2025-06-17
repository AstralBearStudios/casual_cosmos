mod config;

use bevy::prelude::*;
use bevy_common_assets::toml::TomlAssetPlugin;
use config::{Background, Config, ConfigHandle};

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    Loading,
    MainMenu,
    // Settings,
    // ModeSelect,
    // Editor,
    // LevelSelect,
    // Level,
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TomlAssetPlugin::<Config>::new(&[])))
        .insert_state(AppState::Loading)
        .add_systems(Startup, init)
        .add_systems(Update, load_config.run_if(in_state(AppState::Loading)))
        // TODO: replace with buttons. We don't want to watch the file every second!
        .add_systems(Update, load_config.run_if(in_state(AppState::MainMenu)))
        .run();
}

// TODO: replace with String or AssetPath.
const CONFIG_PATH: &str = "tests/background_color.toml";

fn init(mut commands: Commands, asset_server: Res<AssetServer>) {
    let config = ConfigHandle(asset_server.load(CONFIG_PATH));
    commands.insert_resource(config);
    commands.spawn(Camera2d);
}

fn load_config(
    mut commands: Commands,
    assets: ResMut<Assets<Config>>,
    config: Res<ConfigHandle>,
    mut next_state: ResMut<NextState<AppState>>,
    asset_server: Res<AssetServer>,
) {
    if let Some(config) = assets.get(config.0.id()) {
        match &config.background {
            Background::Color(color) => commands.insert_resource(ClearColor(*color)),
            Background::ImagePath(filepath) => {
                commands.spawn(Sprite {
                    image: asset_server.load(filepath.filepath.clone()),
                    ..default()
                });
            }
        }
        next_state.set(AppState::MainMenu);
    }
}
