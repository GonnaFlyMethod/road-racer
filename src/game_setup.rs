use rusty_engine::prelude::*;
use crate::audio_assets::Music;
use crate::common;
use crate::consts;
use crate::game_state::GameState;
use crate::game_loop::game_loop;

fn setup_player(game: &mut Game<GameState>){
    let player_sprite = game.add_sprite(
        consts::PLAYER_LABEL, SpritePreset::RacingCarBlue);

    player_sprite.translation = Vec2::new(consts::X_PLAYER_DEFAULT_POSITION,
                                          consts::Y_PLAYER_DEFAULT_POSITION);
    player_sprite.layer = consts::PLAYER_SPRITE_LAYER;
    player_sprite.collision = true
}

fn setup_roadlines(game: &mut Game<GameState>){
    for i in 0..30 {
        let roadline_label = format!("{}_{}", consts::ROADLINE_LABEL, i);
        let roadline_sprite = game.add_sprite(roadline_label,
                                              SpritePreset::RacingBarrierWhite);

        roadline_sprite.layer = consts::ROADLINE_SPRITE_LAYER;

        roadline_sprite.scale = 0.2;

        roadline_sprite.translation.y = 0.0;
        roadline_sprite.translation.x = (-600 + (150 * i)) as f32 ;
    }
}

fn setup_roadbarries(game: &mut Game<GameState>){
    const ROAD_BARRIER_OFFSET: f32 = 10.0;

    for i in 0..36{
        let barrier_up_label = format!("{}_up{}", consts::ROADBARRIER_LABEL, i);
        let barrier_up_sprite = game.add_sprite(barrier_up_label,
                                                SpritePreset::RacingBarrelRed);

        barrier_up_sprite.layer = consts::BARRIER_SPRITE_LAYER;
        barrier_up_sprite.translation.y = consts::ONE_HALF_OF_ROAD_WIDTH + ROAD_BARRIER_OFFSET;
        barrier_up_sprite.translation.x = (-600 + (52 * i) ) as f32;


        let barrier_down_label = format!("{}_down{}",consts::ROADBARRIER_LABEL, i);
        let barrier_down_sprite = game.add_sprite(barrier_down_label,
                                                  SpritePreset::RacingBarrelRed);

        barrier_down_sprite.layer = consts::BARRIER_SPRITE_LAYER;
        barrier_down_sprite.translation.y = -consts::ONE_HALF_OF_ROAD_WIDTH - ROAD_BARRIER_OFFSET;

        barrier_down_sprite.translation.x = (-600 + (52 * i) ) as f32;
        barrier_down_sprite.rotation = 180.0;
    }
}

fn setup_obstacles(game: &mut Game<GameState>, game_state: &mut GameState){
    // Initial pull of objects

    let initial_num_of_obstacles: u8 = 9;
    let mut obstacle_presets = Vec::with_capacity(initial_num_of_obstacles as usize);

    for _ in 0..initial_num_of_obstacles{
        let random_obstacle_preset = common::gen_random_obstacle_preset();

        obstacle_presets.push(random_obstacle_preset);
    }

    let length_of_obstacle_presets = obstacle_presets.len() as u8;

    for (i, preset) in obstacle_presets.into_iter().enumerate() {
        let obstacle_label = format!("{}_{}",  consts::OBSTACLE_LABEL, i);

        let obstacle_sprite = game.add_sprite(obstacle_label, *preset);

        obstacle_sprite.layer = consts::OBSTACLE_SPRITE_LAYER;
        obstacle_sprite.collision = true;

        obstacle_sprite.translation = common::gen_random_pos_for_obstacle();
    }

    game_state.last_obstacle_index = length_of_obstacle_presets - 1;
}

fn setup_score_text(game: &mut Game<GameState>){
    let score_message = game.add_text(consts::DURING_GAME_SCORE_TEXT_LABEL, "0");

    score_message.translation = Vec2::new(-600.0, 320.0);
    score_message.font_size = 50.0;
}

fn setup_health_text(game: &mut Game<GameState>, initial_game_state: &GameState){
    let initial_health_text = format!("{}{}",
                                      consts::HEALTH_TEXT_TEMPLATE, initial_game_state.health_amount);

    let health_message = game.add_text(consts::HEALTH_TEXT_LABEL, initial_health_text);

    health_message.translation = Vec2::new(550.0, 320.0);
}

pub fn setup_game(game: &mut Game<GameState>, initial_game_state: &mut GameState){
    setup_player(game);
    setup_roadlines(game);
    setup_roadbarries(game);
    setup_obstacles(game, initial_game_state);
    setup_score_text(game);
    setup_health_text(game, initial_game_state);

    game.audio_manager.play_music(Music::Standard, consts::DEFAULT_VOLUME);

    game.add_logic(game_loop);
}