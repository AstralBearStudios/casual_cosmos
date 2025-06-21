use bevy::prelude::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum AppState {
    #[default]
    Loading,
    MainMenu,
    // Settings,
    // ModeSelect,
    // Editor,
    // LevelSelect,
    Level,
}

// Adapted from:
// https://github.com/IDEDARY/Bevypunk
// Under the MIT license
pub fn despawn_scene<S: Component>(mut commands: Commands, query: Query<Entity, With<S>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
