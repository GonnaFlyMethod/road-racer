use crate::consts;

#[derive(Debug)]
pub struct GameState {
    pub road_speed: f32,
    pub health_amount: u8,

    pub num_of_game_progressions_obstacles: u8,
    pub last_second_of_progression_obstacles: u64,

    pub num_of_game_progressions_road_speed: u8,
    pub last_second_of_progression_road_speed: u64,

    pub last_obstacle_index: u8,
    pub lost: bool,

    pub play_session_time: f64
}

impl Default for GameState{
    fn default() -> Self {
        Self{
            road_speed: consts::INITIAL_ROAD_SPEED,
            health_amount: 5,
            num_of_game_progressions_obstacles: 0,
            last_second_of_progression_obstacles: 0,
            num_of_game_progressions_road_speed: 0,
            last_second_of_progression_road_speed: 0,
            last_obstacle_index: 0,
            lost: false,
            play_session_time: 0.0
        }
    }
}
