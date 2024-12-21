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
pub enum SettingsMenu {
    Graphics,
    Audio,
    Controls,
    Back,
    #[default]
    Main,
}
pub fn settings_screen(app: &mut App) {
    app.init_state::<SettingsMenu>()
        .add_systems(OnEnter(Menu::Settings), build_new_game_menu)
        .add_systems(OnExit(Menu::Settings), cleanup::<Menu>)
        .add_plugins(settings_menus)
        .add_plugins((InputManagerPlugin::<SettingsMenu>::default(),))
        .init_resource::<ActionState<SettingsMenu>>()
        .insert_resource(SettingsMenu::default_input_map())
        .add_systems(Update, setting_controls.run_if(in_state(Menu::Settings)));
    //.or(state_changed::<SettingsMenu>)),
}
fn setting_controls(
    input: Res<ActionState<SettingsMenu>>,
    state: Res<State<SettingsMenu>>,
    mut settings_menu: ResMut<NextState<SettingsMenu>>,
    mut menu_state: ResMut<NextState<Menu>>,
) {
    let current = input.get_just_released();
    if current.last() != Some(state.get()) && current.last().is_some() {
        debug!("Settings Menu state: {:?} {:?}", state.get(), current);
        settings_menu.set(*current.last().unwrap_or_else(|| state.get()));
    }
    match state.get() {
        SettingsMenu::Graphics => return,
        SettingsMenu::Audio => return,
        SettingsMenu::Controls => return,
        SettingsMenu::Back => menu_state.set(Menu::Main),
        SettingsMenu::Main => return,
    };
    settings_menu.set(Default::default());
}
fn build_new_game_menu(mut commands: Commands) {
    let mut node = commands.spawn((
        Node {
            left: Val::Percent(60.0),
            top: Val::Percent(60.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        Menu::NewGame,
    ));
    node.with_children(|parent| {
        button(parent, "Controls", None, SettingsMenu::Controls);
        button(parent, "Graphics", None, SettingsMenu::Graphics);
        button(parent, "Audio", None, SettingsMenu::Audio);
        button(parent, "Back", None, SettingsMenu::Back);
    });
}
impl SettingsMenu {
    pub fn default_input_map() -> InputMap<Self> { InputMap::new([(Self::Back, KeyCode::Escape)]) }
}
