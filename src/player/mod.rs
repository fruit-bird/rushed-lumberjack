mod components;
mod resources;
mod systems;

use bevy::prelude::*;
use systems::*;

pub use components::{Health, Player, PlayerBundle};
pub use resources::HealthDrainConfig;
pub use systems::player_is_dead;

pub const PLAYER_VELOCITY: f32 = 500.0;
pub const SPRITE_IDX: usize = 119;
pub const SECONDS_TO_DRAIN_1_HP: f32 = 0.05;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(HealthDrainConfig::new(SECONDS_TO_DRAIN_1_HP))
            .add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement)
            .add_systems(Update, (health_drain, debug_player_hp))
            .add_systems(PreUpdate, player_dies);
    }
}
