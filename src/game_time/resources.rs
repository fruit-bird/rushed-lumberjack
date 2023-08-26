use bevy::prelude::*;
use std::time::Duration;

use super::ROUND_LENGTH;

#[derive(Debug, Resource)]
pub struct GameTime {
    pub secs_remaining: Duration,
}

impl GameTime {
    pub const fn new(secs_remaining: Duration) -> Self {
        Self { secs_remaining }
    }
}

impl Default for GameTime {
    fn default() -> Self {
        Self::new(ROUND_LENGTH)
    }
}
