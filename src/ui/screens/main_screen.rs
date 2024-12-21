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
pub enum MainMenu {
    Continue,
    Play,
    Load,
    Multiplayer,
    Mods,
    Settings,
    Exit,
    #[default]
    Title,
}
pub fn main_menu(app: &mut App) {
    app.init_state::<MainMenu>()
        .add_systems(OnEnter(Menu::Main), build_main_menu)
        .add_systems(OnExit(Menu::Main), cleanup::<Menu>)
        .add_plugins((InputManagerPlugin::<MainMenu>::default(),))
        .init_resource::<ActionState<MainMenu>>()
        .insert_resource(MainMenu::default_input_map())
        .add_systems(Update, keyboard_input_bindings.run_if(in_state(Menu::Main))); //.or(state_changed::<MainMenu>))
    // .add_systems(Update, button_system.run_if(in_state(GameScreen::Main)))
}

fn keyboard_input_bindings(
    input: Res<ActionState<MainMenu>>,
    state: Res<State<MainMenu>>,
    mut main_menu: ResMut<NextState<MainMenu>>,
    mut which_menu: ResMut<NextState<Menu>>,
) {
    let current = input.get_just_released();
    if current.last() != Some(state.get()) && current.last().is_some() {
        warn!("Main Menu state: {:?}", state.get(),);
        main_menu.set(*current.last().unwrap_or_else(|| state.get()));
    }
    match state.get() {
        MainMenu::Continue => which_menu.set(Menu::Interface), // TODO: change when Loading is implemented
        MainMenu::Play => which_menu.set(Menu::NewGame),
        MainMenu::Load => return,
        MainMenu::Multiplayer => unimplemented!(),
        MainMenu::Mods => return,
        MainMenu::Settings => {
            which_menu.set(Menu::Settings);
            return;
        }
        MainMenu::Exit => which_menu.set(Menu::Exiting),
        MainMenu::Title => return,
    };
    main_menu.set(Default::default());
}

fn build_main_menu(mut commands: Commands) {
    const BORDER_SIZE: Val = Val::Px(5.0);
    let mut root = commands.spawn((
        Node {
            left: Val::Percent(30.0),
            right: Val::Percent(60.0),
            top: Val::Percent(50.0),
            bottom: Val::Percent(60.0),
            min_height: Val::Percent(20.0),
            min_width: Val::Percent(20.0),
            max_height: Val::Percent(20.0),
            max_width: Val::Percent(20.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Stretch,
            justify_content: JustifyContent::Center,
            row_gap: BORDER_SIZE,
            border: UiRect::all(BORDER_SIZE),
            ..default()
        },
        Menu::Main,
    ));
    root.with_children(|parent| {
        button(parent, "Continue", None, MainMenu::Continue);
        let mut row = parent.spawn(Node {
            align_items: AlignItems::Stretch,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Row,
            column_gap: BORDER_SIZE,
            ..default()
        });
        row.with_children(|row| {
            button(row, "Load", None, MainMenu::Load);
            let play_color = Some(Color::srgb(0.65, 1.0, 0.65));
            button(row, "Play", play_color, MainMenu::Play);
        });
        button(parent, "Multiplayer", None, MainMenu::Multiplayer);
        button(parent, "Settings", None, MainMenu::Settings);
        button(parent, "Mods", None, MainMenu::Mods);
        let exit_color = Some(Color::srgb(1., 0.60, 0.20));
        button(parent, "Exit", exit_color, MainMenu::Exit);
    });
}

impl MainMenu {
    pub fn default_input_map() -> InputMap<Self> {
        InputMap::new([
            (Self::Continue, KeyCode::Enter),
            (Self::Play, KeyCode::KeyP),
            (Self::Load, KeyCode::KeyL),
            (Self::Settings, KeyCode::KeyS),
            (Self::Exit, KeyCode::Escape),
        ])
    }
}
