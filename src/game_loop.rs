use std::{fs, io};
use serde::{Serialize, Deserialize};
use rusty_engine::prelude::*;
use crate::audio_assets::{Music, SFX};
use crate::common;
use crate::consts;
use crate::game_state::GameState;

fn handle_game_retry(engine:  &mut Engine, game_state: &mut GameState){
    game_state.road_speed = GameState::default().road_speed;
    game_state.health_amount = GameState::default().health_amount;

    game_state.num_of_game_progressions_obstacles = GameState::default().num_of_game_progressions_obstacles;
    game_state.last_second_of_progression_obstacles = GameState::default().last_second_of_progression_obstacles;

    game_state.num_of_game_progressions_road_speed = GameState::default().num_of_game_progressions_road_speed;
    game_state.last_second_of_progression_road_speed = GameState::default().last_second_of_progression_road_speed;

    game_state.last_obstacle_index =  GameState::default().last_obstacle_index;
    game_state.lost = GameState::default().lost;

    game_state.play_session_time = engine.time_since_startup_f64;

    let mut player_sprite = engine.sprites.get_mut(consts::PLAYER_LABEL).unwrap();

    player_sprite.translation = Vec2::new(consts::X_PLAYER_DEFAULT_POSITION,
                                          consts::Y_PLAYER_DEFAULT_POSITION);
    player_sprite.rotation = 0.0;

    // Removing obstacles that are related to game progression
    engine.sprites.retain(
        |label, _| {

            (label.starts_with(consts::OBSTACLE_LABEL) &&
            !label.contains(consts::LABEL_PART_PROGRESSION)) ||
            label.starts_with(consts::PLAYER_LABEL) ||
            label.starts_with(consts::ROADBARRIER_LABEL) ||
            label.starts_with(consts::ROADLINE_LABEL)
        }
    );

    // Rearranging initial obstacles
    for sprite in engine.sprites.values_mut() {
        if sprite.label.starts_with(consts::OBSTACLE_LABEL){
            sprite.translation = common::gen_random_pos_for_obstacle();
        }
    }

    engine.texts.remove(consts::GAMEOVER_TEXT_LABEL);
    engine.texts.remove(consts::SCORE_AND_BEST_SCORE_TEXT_LABEL);
    engine.texts.remove(consts::NEW_RECORD_TEXT_LABEL);
    engine.texts.remove(consts::GAMEOVER_INSTRUCTIONS_TEXT_LABEL);

    engine.audio_manager.play_music(Music::Standard, consts::DEFAULT_VOLUME);
}

fn move_road(engine:  &mut Engine, game_state: &GameState){
    for sprite in engine.sprites.values_mut() {
        if sprite.label.starts_with(consts::ROADLINE_LABEL) {
            sprite.translation.x -= game_state.road_speed * engine.delta_f32;
            if sprite.translation.x < -675.0 {
                sprite.translation.x += 1500.0;
            }
        }

        if sprite.label.starts_with(consts::ROADBARRIER_LABEL){
            sprite.translation.x -= game_state.road_speed * engine.delta_f32;
            if sprite.translation.x < -675.0 {
                sprite.translation.x += 1822.0;
            }
        }

        if sprite.label.starts_with(consts::OBSTACLE_LABEL){
            sprite.translation.x -= game_state.road_speed * engine.delta_f32;

            if sprite.translation.x < -675.0 {
                sprite.translation = common::gen_random_pos_for_obstacle();
            }
        }
    }
}

fn handle_car_control(engine: &mut Engine){
    let mut direction = 0.0;

    let player_sprite = engine.sprites.get_mut(consts::PLAYER_LABEL).unwrap();

    player_sprite.rotation = 0.0;

    if engine.keyboard_state.pressed_any(&[KeyCode::Up, KeyCode::W]){
        direction += 1.0;

        player_sprite.translation.y += direction * consts::PLAYER_SPEED * engine.delta_f32;
        player_sprite.rotation = direction * 0.15;
    } else if engine.keyboard_state.pressed_any(&[KeyCode::Down, KeyCode::S]){
        direction -= 1.0;

        player_sprite.translation.y += direction * consts::PLAYER_SPEED * engine.delta_f32;
        player_sprite.rotation = direction * 0.15;
    }
}

fn handle_collisions(engine: &mut Engine, game_state: &mut GameState){
    for event in engine.collision_events.drain(..) {
        if event.state.is_end(){
            continue
        }

        let col_obj1 = &event.pair.0;
        let col_obj2 = &event.pair.1;

        if col_obj1.starts_with(consts::OBSTACLE_LABEL) &&
            col_obj2.starts_with(consts::OBSTACLE_LABEL){

            let obstacle_to_rearrange = engine.sprites.get_mut(col_obj1).unwrap();

            obstacle_to_rearrange.translation = common::gen_random_pos_for_obstacle();

            continue
        }

        // Handling collision with OnRoad obstacles

       if event.pair.either_contains(consts::PLAYER_LABEL){
           let is_obstacle0 = event.pair.0.contains(consts::OBSTACLE_LABEL);

          let obstacle_tag =if is_obstacle0 { event.pair.0 } else { event.pair.1 };

           let obstacle_to_rearrange = engine.sprites.get_mut(&obstacle_tag).unwrap();

           obstacle_to_rearrange.translation = common::gen_random_pos_for_obstacle();

           engine.audio_manager.play_sfx(SFX::Impact, consts::DEFAULT_VOLUME);

           if game_state.health_amount > 0 {
               game_state.health_amount -= 1;
           }else{
               game_state.health_amount = 0;
           }

           if game_state.health_amount == 0 {
               game_state.lost = true;
               break;
           }
       }
    }
}

fn update_game_state_if_out_of_road(engine: &mut Engine, game_state: &mut GameState){
    let half_of_road_with_offset = consts::ONE_HALF_OF_ROAD_WIDTH - 40.0;

    let player_sprite = engine.sprites.get_mut(consts::PLAYER_LABEL).unwrap();

    if player_sprite.translation.y < -half_of_road_with_offset ||
        player_sprite.translation.y > half_of_road_with_offset {

        engine.audio_manager.play_sfx(SFX::Impact, consts::DEFAULT_VOLUME);

        game_state.health_amount = 0;
        game_state.lost = true;
    }
}

fn update_health_text(engine: &mut Engine, health_amount: u8){
    let updated_health_text = format!(
        "{}{}", consts::HEALTH_TEXT_TEMPLATE, health_amount);

    let health_text = engine.texts.get_mut(consts::HEALTH_TEXT_LABEL).unwrap();
    health_text.value = updated_health_text;
}

#[derive(Serialize, Deserialize, Debug)]
struct GamePersistantData{
    best_score: u64,
}

fn handle_game_over(engine: &mut Engine, score_as_time_elapsed: u64) -> Result<(), io::Error>{
    let mut new_best_score = false;
    let mut best_score: u64 = 0;

    match fs::read(consts::ROAD_RACER_DATA_PATH) {
        Ok(file) =>{
            let game_persistant_data: GamePersistantData = bincode::deserialize(
              file.as_slice()).unwrap();

            best_score = game_persistant_data.best_score;

            if game_persistant_data.best_score < score_as_time_elapsed{
              new_best_score = true;
          }
        },
        Err(err) => {
            if err.kind() == io::ErrorKind::NotFound{
                new_best_score = true;
            } else{
                return Err(err);
            }
        }
    };

    if new_best_score{
        let persistant_data_to_write = &GamePersistantData{
            best_score: score_as_time_elapsed,
        };

        let encoded_persistant_data: Vec<u8> = bincode::serialize(&persistant_data_to_write).unwrap();

        fs::write(consts::ROAD_RACER_DATA_PATH, encoded_persistant_data)?;

        let record_formatted_text = format!("{}{}",
                                            consts::NEW_RECORD_TEXT_TEMPLATE,
                                            score_as_time_elapsed);

        let record_text = engine.add_text(consts::NEW_RECORD_TEXT_LABEL, record_formatted_text);
        record_text.translation.y = -80.0;
    } else {
        let score_and_best_score_formatted_text = format!(
            "Current score: {}. Best score: {}", score_as_time_elapsed, best_score);

        let score_and_best_score_text = engine.add_text(
            consts::SCORE_AND_BEST_SCORE_TEXT_LABEL, score_and_best_score_formatted_text);
        score_and_best_score_text.translation.y = -80.0;
    }

    let game_over_text = engine.add_text(
        consts::GAMEOVER_TEXT_LABEL, consts::GAMEOVER_TEXT_TEMPLATE);
    game_over_text.font_size = 128.0;

    let further_instructions_text = engine.add_text(
        consts::GAMEOVER_INSTRUCTIONS_TEXT_LABEL,
        consts::GAMEOVER_INSTRUCTIONS_TEXT_TEMPLATE);
    further_instructions_text.translation.y = -128.0;

    engine.audio_manager.play_sfx(SFX::GameOver, consts::DEFAULT_VOLUME / 2.5);
    engine.audio_manager.stop_music();

    Ok(())
}

fn update_score_text(engine: &mut Engine, time_elapsed: u64){
    let updated_score_text = format!("{}", time_elapsed);

    let score_text = engine.texts.get_mut(consts::DURING_GAME_SCORE_TEXT_LABEL).unwrap();
    score_text.value = updated_score_text;
}

fn handle_game_progression(engine: &mut Engine, game_state: &mut GameState, time_elapsed: u64){
    if game_state.num_of_game_progressions_obstacles < consts::GAME_PROGRESSIONS_MAX_NUM  &&
        time_elapsed != 0 && time_elapsed % consts::INTERVAL_TO_ADD_NEW_OBSTACLE == 0 &&
        time_elapsed != game_state.last_second_of_progression_obstacles{
            game_state.last_obstacle_index += 1;

            let obstacle_label = format!("{}_{}_{}",  consts::OBSTACLE_LABEL,
                                                            consts::LABEL_PART_PROGRESSION,
                                                            game_state.last_obstacle_index);

            let random_obstacle_preset = common::gen_random_obstacle_preset();

            let obstacle_sprite = engine.add_sprite(obstacle_label,
                                                    *random_obstacle_preset);

            obstacle_sprite.layer = consts::OBSTACLE_SPRITE_LAYER;
            obstacle_sprite.collision = true;

            obstacle_sprite.translation = common::gen_random_pos_for_obstacle();

            game_state.last_second_of_progression_obstacles = time_elapsed;
            game_state.num_of_game_progressions_obstacles += 1;
        }

    if game_state.num_of_game_progressions_road_speed < consts::GAME_PROGRESSIONS_MAX_NUM &&
        time_elapsed != 0 && time_elapsed % consts::INTERVAL_TO_SPEED_UP_ROAD == 0 &&
        time_elapsed != game_state.last_second_of_progression_road_speed{

        game_state.road_speed += consts::ROAD_SPEED_UP_DELTA;

        game_state.last_second_of_progression_road_speed = time_elapsed;
        game_state.num_of_game_progressions_road_speed += 1;
    }
}

pub fn game_loop(engine: &mut Engine, game_state: &mut GameState) {
    if engine.keyboard_state.pressed(KeyCode::Q){
        engine.should_exit = true;
        return;
    }

    if game_state.lost{
        if engine.keyboard_state.pressed(KeyCode::Space){
            handle_game_retry(engine, game_state);
        }

        return;
    }

    let time_elapsed = (engine.time_since_startup_f64 - game_state.play_session_time) as u64;

    update_score_text(engine, time_elapsed);

    handle_game_progression(engine, game_state, time_elapsed);

    move_road(engine, game_state);
    handle_car_control(engine);
    handle_collisions(engine, game_state);

    if !game_state.lost{
        update_game_state_if_out_of_road(engine, game_state);
    }

    update_health_text(engine, game_state.health_amount);

    if game_state.lost{
        let result = handle_game_over(engine, time_elapsed);
        result.expect("`handle_game_over` method should successfully handle filesystem i/o");
    }
}
