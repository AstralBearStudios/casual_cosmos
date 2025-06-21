use bevy::prelude::*;
use bevy::winit::{UpdateMode, WinitSettings};
use std::time::Duration;

pub struct PlatformPlugin;

use crate::base::AppState;

impl Plugin for PlatformPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Casual Cosmos".into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            // set(AssetPlugin {
            //     // TODO: find best option to set this.
            //     // Maybe manually doing this is better?
            //     // watch_for_changes_override: Some(true),
            //     ..default()
            // },),
        )
        .add_systems(Startup, init_refresh_rate)
        .init_state::<AppState>();
    }
}

// Adapted from:
// https://rustunit.com/blog/2025/01-02-bevy-mobile-framerate/
fn init_refresh_rate(mut winit: ResMut<WinitSettings>) {
    // TODO: maybe set FPS as user-configurable?
    // The initial value should be small on startup.
    const FPS: f32 = 60.0;
    winit.focused_mode = UpdateMode::reactive(Duration::from_secs_f32(1.0 / FPS));
}
