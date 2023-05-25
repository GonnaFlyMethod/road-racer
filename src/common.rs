use rand::{Rng, thread_rng};
use rusty_engine::prelude::Vec2;
use rusty_engine::sprite::SpritePreset;
use crate::consts;

pub fn gen_random_pos_for_obstacle() -> Vec2 {
    let x = thread_rng()
        .gen_range(consts::X_OBSTACLE_RAND_RANGE_START..consts::X_OBSTACLE_RAND_RANGE_END);

    let y = thread_rng()
        .gen_range(consts::Y_OBSTACLE_RAND_RANGE_START..consts::Y_OBSTACLE_RAND_RANGE_END);

    Vec2::new(x, y)
}

pub fn gen_random_obstacle_preset<'a>() -> &'a SpritePreset{
    let random_index = thread_rng()
        .gen_range(0..consts::GAME_OBSTACLE_PRESETS.len());

    &consts::GAME_OBSTACLE_PRESETS[random_index]
}
