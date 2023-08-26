mod components;
mod events;
mod resources;
mod systems;

use crate::game_state::AppState;

use bevy::{prelude::*, time::common_conditions::on_fixed_timer};
use std::time::Duration;
use systems::*;

pub use components::{Health, Player, PlayerBundle};
pub use events::PlayerDied;
pub use resources::HealthDrainConfig;

pub const PLAYER_VELOCITY: f32 = 500.0;
pub const SPRITE_IDX: usize = 119;
pub const SECONDS_TO_DRAIN_1_HP: f32 = 0.5;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(HealthDrainConfig::new(SECONDS_TO_DRAIN_1_HP))
            .add_event::<PlayerDied>()
            .add_systems(Startup, spawn_player.run_if(in_state(AppState::Playing)))
            .add_systems(
                Update,
                (player_movement, health_drain, player_dies)
                    .distributive_run_if(in_state(AppState::Playing)),
            )
            .add_systems(
                FixedUpdate,
                debug_player_hp.run_if(on_fixed_timer(Duration::from_secs(1))),
            );
    }
}
