use super::*;
use bevy::{app::AppExit, state::state::FreelyMutableState};

pub mod ingame_menu;
pub mod ingame_screen;
pub mod main_screen;
pub mod new_game_screen;
pub mod settings_menu;
pub mod settings_screen;

pub use ingame_menu::*;
pub use ingame_screen::*;
pub use main_screen::*;
pub use new_game_screen::*;
pub use settings_menu::*;
pub use settings_screen::*;

const DEFAULT_MENU_COLOR: Color = Color::srgb(0.75, 0.75, 0.75);

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
pub enum Menu {
    #[default]
    Main,
    NewGame,
    InGameMenu,
    Interface,
    Settings,
    Loading,
    Exiting,
}
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States, Resource)]
pub enum IsPaused {
    #[default]
    No,
    Yes,
}
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States, Resource)]
pub enum Confirm {
    #[default]
    No,
    Yes,
}
pub struct ScreensPlugin;
impl Plugin for ScreensPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup)
            .init_state::<Menu>()
            .init_state::<IsPaused>()
            .init_state::<Confirm>()
            .add_plugins(main_menu)
            .add_plugins(new_game)
            .add_plugins(ingame_screen)
            .add_plugins(ingame_menu)
            .add_plugins(settings_screen)
            .add_plugins(InputManagerPlugin::<Menu>::default())
            .init_resource::<ActionState<Menu>>()
            .insert_resource(Menu::default_input_map())
            .add_systems(OnEnter(Menu::Exiting), exit_system);
    }
}

fn startup(mut commands: Commands) { commands.spawn(Camera2d); }
fn exit_system(mut exit: EventWriter<AppExit>) { exit.send(AppExit::Success); }
fn button<T: Copy + Component + States + FreelyMutableState>(
    parent: &mut ChildBuilder<'_>,
    name: &str,
    background_color: Option<Color>,
    btname: T,
) {
    // let n = move || btname;
    parent
        .spawn((
            Button,
            Text::new(name),
            TextFont {
                font_size: 24.,
                ..default()
            },
            TextColor(Color::BLACK),
            BorderColor(Color::BLACK),
            BackgroundColor(background_color.unwrap_or(DEFAULT_MENU_COLOR)),
            btname,
        ))
        .observe(
            move |_click: Trigger<Pointer<Click>>, mut next: ResMut<NextState<T>>| {
                next.set(btname);
            },
        );
}
