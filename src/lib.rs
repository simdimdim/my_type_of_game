#![allow(clippy::type_complexity)]

pub use anyhow::Result;
pub use bevy::prelude::*;
#[cfg(target_os = "macos")]
use bevy::window::CompositeAlphaMode;
use bevy::{
    log::{Level, LogPlugin},
    window::{ExitCondition, PresentMode, WindowLevel, WindowMode},
};
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
pub use leafwing_input_manager::prelude::*;

pub mod ui;

pub use ui::*;

pub static APPNAME: &str = "My Type of Game";

pub struct GamePlugins;

impl Plugin for GamePlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: APPNAME.into(),
                        present_mode: PresentMode::AutoVsync,
                        transparent: true,
                        decorations: true,
                        resizable: true,
                        mode: WindowMode::Fullscreen(MonitorSelection::Primary),
                        #[cfg(target_os = "macos")]
                        composite_alpha_mode: CompositeAlphaMode::PostMultiplied,
                        window_level: WindowLevel::AlwaysOnTop,
                        ..default()
                    }),
                    exit_condition: ExitCondition::OnPrimaryClosed,
                    close_when_requested: true,
                })
                .set(LogPlugin {
                    level: Level::WARN,
                    filter: "warn,log_layers_ecs=warn".to_string(),
                    ..default()
                }),
        )
        .insert_resource(ClearColor(Color::NONE))
        .add_plugins(UiPlugin);
        // .add_plugins(WorldInspectorPlugin::new())
        // .add_systems(Update, click_through);
    }
}
// fn click_through(hovering: EventReader<Pointer<Over>>, mut window: Single<&mut Window>) {
//     let passthrough = window.cursor_options.hit_test;
//     window.cursor_options.hit_test = !hovering.is_empty();
//     warn!("not passing true?: {passthrough:?}");
// }
// fn s(mut window: Single<&mut Window>) {
//     warn!("setting to click-through");
//     window.cursor_options.hit_test = false;
// }
