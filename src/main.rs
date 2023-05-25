use rusty_engine::prelude::*;
use crate::game_setup::setup_game;
use crate::game_state::GameState;

mod game_setup;
mod consts;
mod game_state;
mod game_loop;
mod common;
mod audio_assets;

// TODO:
// 1) Game tutorial

fn main() {
    let mut game = Game::new();

    game.window_settings(WindowDescriptor{
        title: "Road racer".to_string(),
        resizable: false,
        ..WindowDescriptor::default()
    });

    let mut initial_game_state = GameState::default();

    setup_game(&mut game, &mut initial_game_state);

    game.run(initial_game_state);
}
