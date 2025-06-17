use bevy::prelude::*;

fn main() {
    App::new()
        // TODO: replace with DefaultPlugins
        // once loading the Config works.
        .add_plugins((TaskPoolPlugin { ..default() }, AssetPlugin { ..default() }))
        .run();
}
