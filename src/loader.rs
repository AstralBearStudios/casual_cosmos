use crate::base::AppState;
use crate::config::{Config, ConfigHandle};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_common_assets::toml::TomlAssetPlugin;

pub struct LoaderPlugin;

impl Plugin for LoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TomlAssetPlugin::<Config>::new(&["toml"]))
            .add_systems(Startup, setup_loading)
            .add_loading_state(
                LoadingState::new(AppState::Loading)
                    .continue_to_state(AppState::Level)
                    .load_collection::<ConfigHandle>(), // TODO: make dynamic loading work!
                                                        // .register_dynamic_asset_collection::<Config>()
                                                        // .with_dynamic_assets_file::<Config>("backgrond_color.toml"),
            );
    }
}

#[derive(Component)]
pub struct LoadingTag;

fn setup_loading(mut commands: Commands) {
    commands.spawn(Camera2d);
    // commands.spawn((Camera2d, UiSourceCamera::<0>));
    //
    // commands
    //     .spawn((LoadingTag, UiLayoutRoot::new_2d(), UiFetchFromCamera::<0>))
    //     .with_children(|ui| {
    //         ui.spawn((
    //             UiLayout::window()
    //                 .anchor(Anchor::Center)
    //                 .pos(Rl((50.0, 50.0)))
    //                 .size(Rl((50.0, 50.0)))
    //                 .pack(),
    //             UiColor::from(Color::srgb(1.0, 0.0, 0.0)),
    //         ))
    //         .with_children(|ui| {
    //             ui.spawn((
    //                 UiLayout::window()
    //                     .pos(Rl(50.0))
    //                     .anchor(Anchor::Center)
    //                     .pack(),
    //                 UiDepth::Add(5.0),
    //                 UiTextSize::from(Rh(10.0)),
    //                 Text2d::new("Loading..."),
    //             ));
    //         });
    //     });
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
