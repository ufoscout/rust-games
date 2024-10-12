use bevy::prelude::*;
use plugin::MyPlugin;
use resources::{Game, GameState};

mod components;
mod constants;
mod plugin;
mod resources;
mod setup;
mod systems;
mod utils;
 
fn main() {
    App::new()
    // Add a global resource that holds the game state
    .init_resource::<Game>()

    // System that runs once at the start of the app
    .add_systems(Startup, setup::setup)

    // Systems that run every frame
    .add_systems(Update, systems::blink_space_bar_text.run_if(is_game_not_active))
    .add_systems(Update, systems::move_background.run_if(is_game_active))
    .add_systems(Update, systems::move_ground.run_if(is_game_active))
    .add_systems(Update, systems::animate_bird.run_if(is_game_active))
    .add_systems(Update, systems::start_game.run_if(is_game_not_active))
    .add_systems(Update, systems::gravity.run_if(is_game_active))
    .add_systems(Update, systems::jump.run_if(is_game_active))
    .add_systems(Update, systems::pipes.run_if(is_game_active))
    .add_systems(Update, systems::score.run_if(is_game_active))
    .add_systems(Update, systems::render_score.run_if(is_game_active))

    .add_plugins(MyPlugin).run();
}

fn is_game_active(game: Res<Game>) -> bool {
    game.state == GameState::Active
}
 
fn is_game_not_active(game: Res<Game>) -> bool {
    game.state != GameState::Active
}