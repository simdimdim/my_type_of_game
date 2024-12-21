use super::*;

pub mod screens;

pub use screens::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) { app.add_plugins(ScreensPlugin); }
}

fn cleanup<T>(query: Query<Entity, With<T>>, mut commands: Commands)
where
    T: Component, {
    for item in query.iter() {
        commands.entity(item).despawn_recursive();
    }
}
