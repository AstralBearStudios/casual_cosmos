// mod hidden_object;
// mod time_management;

use crate::base::AppState;
use bevy::prelude::*;

pub struct LevelPlugin;

// TODO: refactor into separate game types
impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Level), setup_level)
            .add_systems(Update, animate_sprite.run_if(in_state(AppState::Level)));
    }
}

#[derive(Component)]
struct BarrelTag;

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
    AtHome,
    Active,
    Passive,
}

fn setup_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    // loading_query: Query<Entity, With<LoadingTag>>,
) {
    let barrel = asset_server.load("bevy/rpg/props/generic-rpg-barrel01.png");
    commands
        .spawn((
            BarrelTag,
            Sprite {
                image: barrel,
                ..default()
            },
            Transform::from_scale(Vec3::splat(6.0)),
            Pickable::default(),
        ))
        .observe(check_click);

    let home = asset_server.load("bevy/rpg/chars/vendor/generic-rpg-vendor.png");
    let home_transform = Transform::from_xyz(-300.0, 0.0, 0.0);
    commands
        .spawn((
            BarrelTag,
            Sprite {
                image: home,
                ..default()
            },
            home_transform.with_scale(Vec3::splat(6.0)),
            // Pickable::default(),
        ))
        .observe(check_click);

    // Adapted from:
    // https://bevy.org/examples/picking/sprite-picking/
    let gabe = asset_server.load("bevy/rpg/chars/gabe/gabe-idle-run.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(24, 24), 7, 1, None, None);
    let texture_atlas_layout_handle = texture_atlas_layouts.add(layout);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { first: 1, last: 6 };
    commands.spawn((
        WorkerTag,
        WorkerState::Passive,
        Sprite::from_atlas_image(
            gabe,
            TextureAtlas {
                layout: texture_atlas_layout_handle,
                index: animation_indices.first,
            },
        ),
        Visibility::Hidden,
        home_transform.with_scale(Vec3::splat(6.0)),
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Pickable::default(),
    ));
}

fn check_click(_click: Trigger<Pointer<Pressed>>, mut query: Query<&mut WorkerState>) {
    for mut state in &mut query {
        match *state {
            WorkerState::AtHome | WorkerState::Passive => *state = WorkerState::Active,
            _ => (),
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
            _ => (),
        }
    }
}
