use super::*;

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    Hash,
    PartialEq,
    Actionlike,
    Component,
    States,
    Resource,
    Reflect,
)]
pub enum ControlsMenu {
    Graphics,
    Audio,
    Controls,
    #[default]
    Back,
}
pub fn settings_menus(app: &mut App) {
    app.add_systems(OnEnter(SettingsMenu::Controls), controls_menu)
        .add_systems(OnExit(SettingsMenu::Controls), cleanup::<SettingsMenu>)
        .add_systems(OnEnter(SettingsMenu::Graphics), gfx_menu)
        .add_systems(OnExit(SettingsMenu::Graphics), cleanup::<SettingsMenu>)
        .add_systems(OnEnter(SettingsMenu::Audio), sonic_menu)
        .add_systems(OnExit(SettingsMenu::Audio), cleanup::<SettingsMenu>);
}
fn gfx_menu() {}
fn sonic_menu() {}
fn controls_menu() {}
