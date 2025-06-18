mod config;

use bevy::prelude::*;
use bevy::winit::{UpdateMode, WinitSettings};
use bevy_asset_loader::prelude::*;
use bevy_common_assets::toml::TomlAssetPlugin;
use bevy_lunex::prelude::*;
use config::{Background, Config, ConfigHandle};
use std::time::Duration;

// Adapted from:
// https://github.com/IDEDARY/Bevypunk
// Under the MIT license
fn despawn_scene<S: Component>(mut commands: Commands, query: Query<Entity, With<S>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum AppState {
    #[default]
    Loading,
    MainMenu,
    // Settings,
    // ModeSelect,
    // Editor,
    // LevelSelect,
    Level,
}

fn init_refresh_rate(mut winit: ResMut<WinitSettings>) {
    const FPS: f32 = 60.0;
    winit.focused_mode = UpdateMode::reactive(Duration::from_secs_f32(1.0 / FPS));
}

fn main() {
    App::new()
        .add_plugins((
            // DefaultPlugins.set(AssetPlugin {
            //     // TODO: find best option to set this.
            //     // Maybe manually doing this is better?
            //     // watch_for_changes_override: Some(true),
            //     ..default()
            // },),
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Casual Cosmos".into(),
                    ..default()
                }),
                ..default()
            }),
            TomlAssetPlugin::<Config>::new(&["toml"]),
            UiLunexPlugins,
        ))
        .init_state::<AppState>()
        // Adapted from:
        // https://rustunit.com/blog/2025/01-02-bevy-mobile-framerate/
        .add_systems(Startup, (init_refresh_rate, setup_loading))
        .add_loading_state(
            LoadingState::new(AppState::Loading)
                .continue_to_state(AppState::MainMenu)
                .load_collection::<ConfigHandle>(), // TODO: make dynamic loading work!
                                                    // .register_dynamic_asset_collection::<Config>()
                                                    // .with_dynamic_assets_file::<Config>("backgrond_color.toml"),
        )
        .add_systems(OnEnter(AppState::MainMenu), setup_config)
        // .add_systems(OnEnter(AppState::Level), test)
        // .add_systems(Update, load_config.run_if(in_state(AppState::Loading)))
        // .add_systems(Update, setup_config.run_if(in_state(AppState::MainMenu)))
        // TODO: replace with buttons. We don't want to watch the file every second!
        // TODO: remove previous *background* images! Need a tag for these!
        // .add_systems(Update, load_config.run_if(in_state(AppState::MainMenu)))
        .run();
}

#[derive(Component)]
struct LoadingTag;

fn setup_loading(mut commands: Commands) {
    commands.spawn((Camera2d, UiSourceCamera::<0>));

    // Add a loading screen
    commands
        .spawn((LoadingTag, UiLayoutRoot::new_2d(), UiFetchFromCamera::<0>))
        .with_children(|ui| {
            ui.spawn((
                UiLayout::window()
                    .anchor(Anchor::Center)
                    .pos(Rl((50.0, 50.0)))
                    .size(Rl((50.0, 50.0)))
                    .pack(),
                UiColor::from(Color::srgb(1.0, 0.0, 0.0)),
            ))
            .with_children(|ui| {
                ui.spawn((
                    UiLayout::window()
                        .pos(Rl(50.0))
                        .anchor(Anchor::Center)
                        .pack(),
                    UiDepth::Add(5.0),
                    // This controls the height of the text, so 10% of the parent's node height
                    UiTextSize::from(Rh(10.0)),
                    // Set the starting text value
                    Text2d::new("Loading..."),
                    // Set the text animation
                    // TextAnimator::new("Hello 2D UI!"),
                ));
            });
        });
}

#[derive(Component)]
struct BackgroundTag;

fn setup_config(
    mut commands: Commands,
    assets: ResMut<Assets<Config>>,
    asset_server: Res<AssetServer>,
    config: Res<ConfigHandle>,
    loading_query: Query<Entity, With<LoadingTag>>,
) {
    if let Some(config) = assets.get(config.inner.id()) {
        match &config.background {
            Background::Color(color) => commands.insert_resource(ClearColor(*color)),
            Background::ImagePath(filepath) => {
                commands.spawn((
                    BackgroundTag,
                    Sprite {
                        image: asset_server.load(filepath.filepath.clone()),
                        ..default()
                    },
                ));
            }
        }
    }

    for entity in loading_query.iter() {
        commands.entity(entity).despawn();
    }
}
