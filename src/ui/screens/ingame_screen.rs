use super::*;

pub fn ingame_screen(app: &mut App) {
    // .add_systems(OnEnter(GScreen::InGame), build_ingame_menu)
    app.add_systems(OnExit(Menu::Interface), cleanup::<Menu>)
        .add_systems(Update, ingame_controls.run_if(in_state(Menu::Interface)))
        .add_systems(
            Update,
            char_ctrl.run_if(in_state(Menu::Interface).and(in_state(IsPaused::No))),
        );
    // .or(state_changed::<PlayerUI>)
}
fn char_ctrl() {}
fn ingame_controls(
    input: Res<ActionState<Menu>>,
    mut next_state: ResMut<NextState<Menu>>,
    mut paused: ResMut<NextState<IsPaused>>,
) {
    if input.just_released(&Menu::InGameMenu) {
        paused.set(IsPaused::Yes);
        next_state.set(Menu::InGameMenu);
    }
}
impl Menu {
    pub fn default_input_map() -> InputMap<Self> {
        InputMap::new([(Self::InGameMenu, KeyCode::Escape)])
    }
}
