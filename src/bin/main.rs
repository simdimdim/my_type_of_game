use my_type_of_game::*;

fn main() -> Result<()> {
    let mut game = bevy::app::App::new();
    game.add_plugins(GamePlugins);
    game.run();
    Ok(())
}
