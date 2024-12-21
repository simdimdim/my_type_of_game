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
    Resource,
    States,
    Reflect,
)]
pub enum NewGameMenu {
    Start,
    Back,
    #[default]
    Configuring,
}
pub fn new_game(app: &mut App) {
    app.init_state::<NewGameMenu>()
        .add_systems(OnEnter(Menu::NewGame), build_new_game_menu)
        .add_systems(OnExit(Menu::NewGame), cleanup::<Menu>)
        .add_plugins((InputManagerPlugin::<NewGameMenu>::default(),))
        .init_resource::<ActionState<NewGameMenu>>()
        .insert_resource(NewGameMenu::default_input_map())
        .add_systems(Update, new_game_controls.run_if(in_state(Menu::NewGame)));
}
fn new_game_controls(
    input: Res<ActionState<NewGameMenu>>,
    state: Res<State<NewGameMenu>>,
    mut new_game_menu: ResMut<NextState<NewGameMenu>>,
    mut menu_state: ResMut<NextState<Menu>>,
) {
    let current = input.get_just_released();
    if current.last() != Some(state.get()) && current.last().is_some() {
        warn!("New Game state: {:?}", state.get(),);
        new_game_menu.set(*current.last().unwrap_or_else(|| state.get()));
    }
    match state.get() {
        NewGameMenu::Configuring => return,
        NewGameMenu::Start => menu_state.set(Menu::Interface),
        NewGameMenu::Back => menu_state.set(Menu::Main),
    };
    new_game_menu.set(Default::default());
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
        button(parent, "PLAY", None, NewGameMenu::Start);
        button(parent, "Back", None, NewGameMenu::Back);
    });
}
impl NewGameMenu {
    pub fn default_input_map() -> InputMap<Self> {
        InputMap::new([(Self::Start, KeyCode::Enter), (Self::Back, KeyCode::Escape)])
    }
}
