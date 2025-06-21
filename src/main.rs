mod base;
mod config;
mod platform;

use base::AppState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_common_assets::toml::TomlAssetPlugin;
use bevy_lunex::prelude::*;
use config::{Background, Config, ConfigHandle};
use platform::PlatformPlugin;

fn main() {
    App::new()
        .add_plugins((
            PlatformPlugin,
            TomlAssetPlugin::<Config>::new(&["toml"]),
            UiLunexPlugins,
        ))
        // Adapted from:
        // https://rustunit.com/blog/2025/01-02-bevy-mobile-framerate/
        .add_systems(Startup, setup_loading)
        .add_loading_state(
            LoadingState::new(AppState::Loading)
                .continue_to_state(AppState::Level)
                .load_collection::<ConfigHandle>(), // TODO: make dynamic loading work!
                                                    // .register_dynamic_asset_collection::<Config>()
                                                    // .with_dynamic_assets_file::<Config>("backgrond_color.toml"),
        )
        .add_systems(OnEnter(AppState::Level), setup_config)
        // TODO: temp
        .add_systems(Update, animate_sprite.run_if(in_state(AppState::Level)))
        //
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
                    UiTextSize::from(Rh(10.0)),
                    Text2d::new("Loading..."),
                ));
            });
        });
}

#[derive(Component)]
struct BackgroundTag;

#[derive(Component)]
struct WorkerTag;

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
enum WorkerState {
    Active,
    Passive,
}

fn setup_config(
    mut commands: Commands,
    assets: ResMut<Assets<Config>>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
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

    // NOTE: this is for testing mechanics
    //
    // Adapted from:
    // https://bevy.org/examples/picking/sprite-picking/
    let layout = TextureAtlasLayout::from_grid(UVec2::new(24, 24), 7, 1, None, None);
    let texture_atlas_layout_handle = texture_atlas_layouts.add(layout);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { first: 1, last: 6 };
    commands
        .spawn((
            WorkerTag,
            WorkerState::Passive,
            Sprite::from_atlas_image(
                config.worker.clone(),
                TextureAtlas {
                    layout: texture_atlas_layout_handle,
                    index: animation_indices.first,
                },
            ),
            Transform::from_xyz(300.0, 0.0, 0.0).with_scale(Vec3::splat(6.0)),
            animation_indices,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            Pickable::default(),
        ))
        .observe(check_click);
}

fn check_click(_click: Trigger<Pointer<Pressed>>, mut query: Query<&mut WorkerState>) {
    for mut state in &mut query {
        match *state {
            WorkerState::Passive => *state = WorkerState::Active,
            WorkerState::Active => *state = WorkerState::Passive,
        }
    }
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &WorkerState,
        &mut Sprite,
    )>,
) {
    for (indices, mut timer, worker_state, mut sprite) in &mut query {
        let Some(texture_atlas) = &mut sprite.texture_atlas else {
            continue;
        };

        timer.tick(time.delta());

        match worker_state {
            WorkerState::Passive => texture_atlas.index = 0,
            WorkerState::Active => {
                if timer.just_finished() {
                    texture_atlas.index = if texture_atlas.index == indices.last {
                        indices.first
                    } else {
                        texture_atlas.index + 1
                    };
                }
            }
        }
    }
}
