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
        .add_systems(Update, load_background.run_if(in_state(AppState::Loading)))
        .run();
}

fn init(mut commands: Commands, asset_server: Res<AssetServer>) {
    let config = ConfigHandle(asset_server.load("tests/background_color.toml"));
    commands.insert_resource(config);
    // commands.insert_resource(ClearColor(Color::srgb(1., 1., 1.)));

    commands.spawn(Camera2d);
}

fn load_background(
    mut commands: Commands,
    assets: ResMut<Assets<Config>>,
    config: Res<ConfigHandle>,
    // state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if let Some(config) = assets.get(config.0.id()) {
        match config.background {
            Background::Color(color) => commands.insert_resource(ClearColor(color)),
            // _ => {}
        }
        next_state.set(AppState::MainMenu);
    }
}
