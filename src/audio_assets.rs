pub enum Music {
    Standard
}

impl From<Music> for String {
    fn from(music_preset: Music) -> String {
        match music_preset {
            Music::Standard => "music/background_music.ogg".into()
        }
    }
}

pub enum SFX {
    Impact,
    GameOver
}

impl From<SFX> for String {
    fn from(sfx_preset: SFX) -> Self {
        match sfx_preset {
            SFX::Impact => "sfx/impact.ogg".into(),
            SFX::GameOver => "sfx/game_over.ogg".into()
        }
    }
}