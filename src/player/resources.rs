use bevy::prelude::*;

use super::SECONDS_TO_DRAIN_1_HP;

#[derive(Debug, Resource)]
pub struct HealthDrainConfig {
    pub timer: Timer,
}

impl HealthDrainConfig {
    pub fn new(seconds: f32) -> Self {
        Self {
            timer: Timer::from_seconds(seconds, TimerMode::Repeating),
        }
    }
}

impl Default for HealthDrainConfig {
    fn default() -> Self {
        Self::new(SECONDS_TO_DRAIN_1_HP)
    }
}
