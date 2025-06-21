mod base;
mod config;
mod loader;
mod platform;

use base::AppState;
use bevy::prelude::*;
use bevy_lunex::prelude::*;
use loader::LoaderPlugin;
use platform::PlatformPlugin;

fn main() {
    App::new()
        .add_plugins((PlatformPlugin, LoaderPlugin, UiLunexPlugins))
        // FIXME: only use for testing levels
        .insert_state(AppState::Level)
        .run();
}

// fn check_click(_click: Trigger<Pointer<Pressed>>, mut query: Query<&mut WorkerState>) {
//     for mut state in &mut query {
//         match *state {
//             WorkerState::Passive => *state = WorkerState::Active,
//             WorkerState::Active => *state = WorkerState::Passive,
//         }
//     }
// }
//
// fn animate_sprite(
//     time: Res<Time>,
//     mut query: Query<(
//         &AnimationIndices,
//         &mut AnimationTimer,
//         &WorkerState,
//         &mut Sprite,
//     )>,
// ) {
//     for (indices, mut timer, worker_state, mut sprite) in &mut query {
//         let Some(texture_atlas) = &mut sprite.texture_atlas else {
//             continue;
//         };
//
//         timer.tick(time.delta());
//
//         match worker_state {
//             WorkerState::Passive => texture_atlas.index = 0,
//             WorkerState::Active => {
//                 if timer.just_finished() {
//                     texture_atlas.index = if texture_atlas.index == indices.last {
//                         indices.first
//                     } else {
//                         texture_atlas.index + 1
//                     };
//                 }
//             }
//         }
//     }
// }
