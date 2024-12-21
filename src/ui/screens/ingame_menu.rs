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
pub enum GameMenu {
    Pause,
    Continue,
    Save,
    Load,
    Settings,
    Back,
    #[default]
    Playing,
}
pub fn ingame_menu(app: &mut App) {
    app.init_state::<GameMenu>()
        .add_systems(OnEnter(Menu::InGameMenu), build_ingame_menu)
        .add_systems(OnExit(Menu::InGameMenu), cleanup::<Menu>)
        .add_plugins((InputManagerPlugin::<GameMenu>::default(),))
        .init_resource::<ActionState<GameMenu>>()
        .insert_resource(GameMenu::default_input_map())
        .add_systems(
            Update,
            ingame_menu_controls.run_if(in_state(Menu::InGameMenu)), //.or(state_changed::<GameMenu>)),
        );
}

fn ingame_menu_controls(
    input: Res<ActionState<GameMenu>>,
    state: Res<State<GameMenu>>,
    mut game_menu: ResMut<NextState<GameMenu>>,
    mut set_pause: ResMut<NextState<IsPaused>>,
    mut which_menu: ResMut<NextState<Menu>>,
) {
    let current = input.get_just_released();
    if current.last() != Some(state.get()) && current.last().is_some() {
        debug!("InGame Menu state: {:?} {:?}", state.get(), current);
        game_menu.set(*current.last().unwrap_or_else(|| state.get()));
    }
    match state.get() {
        GameMenu::Playing => return,
        GameMenu::Continue => {
            which_menu.set(Menu::Interface);
            set_pause.set(IsPaused::No);
        }
        GameMenu::Pause => {
            set_pause.set(IsPaused::Yes);
            which_menu.set(Menu::Interface);
        }
        GameMenu::Save => return,
        GameMenu::Load => return,
        GameMenu::Settings => return,
        GameMenu::Back => {
            which_menu.set(Menu::default());
            set_pause.set(IsPaused::default());
        }
    };
    game_menu.set(Default::default());
}
fn build_ingame_menu(mut commands: Commands) {
    const BORDER_SIZE: Val = Val::Px(5.0);
    let mut root = commands.spawn((
        Node {
            left: Val::Percent(40.0),
            right: Val::Percent(60.0),
            top: Val::Percent(40.0),
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
        Menu::InGameMenu,
    ));
    root.with_children(|parent| {
        button(parent, "Continue", None, GameMenu::Continue);
        button(parent, "Pause", None, GameMenu::Pause);
        let mut row = parent.spawn(Node {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Stretch,
            flex_direction: FlexDirection::Row,
            column_gap: BORDER_SIZE,
            ..default()
        });
        row.with_children(|row| {
            button(row, "Load", None, GameMenu::Load);
            let play_color = Some(Color::srgb(0.65, 1.0, 0.65));
            button(row, "Save", play_color, GameMenu::Save);
        });
        button(parent, "Settings", None, GameMenu::Settings);
        let exit_color = Some(Color::srgb(1., 0.60, 0.20));
        button(parent, "Main menu", exit_color, GameMenu::Back);
    });
}
impl GameMenu {
    pub fn default_input_map() -> InputMap<Self> {
        InputMap::new([
            (Self::Continue, KeyCode::Escape),
            (Self::Pause, KeyCode::Space),
            (Self::Save, KeyCode::F5),
            (Self::Load, KeyCode::F9),
            (Self::Settings, KeyCode::F10),
            (Self::Back, KeyCode::F12),
        ])
    }
}
