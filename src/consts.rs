use rusty_engine::sprite::SpritePreset;

// Labels
pub const PLAYER_LABEL: &str = "player";
pub const ROADLINE_LABEL: &str = "roadline";
pub const ROADBARRIER_LABEL: &str = "road_barrier";
pub const OBSTACLE_LABEL: &str = "obstacle";
pub const HEALTH_TEXT_LABEL: &str = "health_text";
pub const DURING_GAME_SCORE_TEXT_LABEL: &str = "during_game_score_text";
pub const GAMEOVER_TEXT_LABEL: &str = "game_over_text";
pub const SCORE_AND_BEST_SCORE_TEXT_LABEL: &str = "score_and_best_score_label";
pub const NEW_RECORD_TEXT_LABEL: &str = "new_record_text";
pub const GAMEOVER_INSTRUCTIONS_TEXT_LABEL: &str = "game_over_instructions_text";

// Text templates
pub const HEALTH_TEXT_TEMPLATE: &str = "Health: ";
pub const GAMEOVER_TEXT_TEMPLATE: &str = "Game Over";
pub const GAMEOVER_INSTRUCTIONS_TEXT_TEMPLATE: &str = "Press Space to retry or Q to exit";
pub const NEW_RECORD_TEXT_TEMPLATE: &str = "New record! ";

// Sprite layers
pub const ROADLINE_SPRITE_LAYER: f32 = 1.0;
pub const BARRIER_SPRITE_LAYER: f32 = 1.0;
pub const OBSTACLE_SPRITE_LAYER: f32 = 2.0;
pub const PLAYER_SPRITE_LAYER: f32 = 3.0;

// InGame Numeric constants
pub const ONE_HALF_OF_ROAD_WIDTH: f32 = 360.0;

pub const DEFAULT_VOLUME: f32 = 0.5;

pub const X_PLAYER_DEFAULT_POSITION: f32 = -500.0;
pub const Y_PLAYER_DEFAULT_POSITION: f32 = 0.0;

pub const X_OBSTACLE_RAND_RANGE_START: f32 = 800.0;
pub const X_OBSTACLE_RAND_RANGE_END: f32 = 2000.0;

pub const Y_OBSTACLE_RAND_RANGE_START: f32 = -300.0;
pub const Y_OBSTACLE_RAND_RANGE_END: f32 = 300.0;

pub const PLAYER_SPEED: f32 = 300.0;
pub const INITIAL_ROAD_SPEED: f32 = 400.0;

// Game progression (Time is set in seconds)
pub const LABEL_PART_PROGRESSION: &str = "progression";

pub const INTERVAL_TO_ADD_NEW_OBSTACLE: u64 = 15;

pub const INTERVAL_TO_SPEED_UP_ROAD: u64 = 20;

pub const ROAD_SPEED_UP_DELTA: f32 = 20.0;

// Specifies how many times progression can happen in the game
pub const GAME_PROGRESSIONS_MAX_NUM: u8 = 10;

// Presets
pub const GAME_OBSTACLE_PRESETS: [&'static SpritePreset; 5] = [
    &SpritePreset::RacingBarrelBlue,
    &SpritePreset::RacingConeStraight,
    &SpritePreset::RollingBallBlueAlt,
    &SpritePreset::RollingBlockSquare,
    &SpritePreset::RollingBlockSmall
];

// File system
pub const ROAD_RACER_DATA_PATH: &str = "./road-racer-data.bin";
