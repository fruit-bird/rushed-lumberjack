mod events;
mod resources;
mod systems;

use bevy::{prelude::*, time::common_conditions::on_fixed_timer};
use std::time::Duration;
use systems::*;

pub use events::GameEnded;
pub use resources::GameTime;

use crate::game_state::AppState;

pub const ROUND_LENGTH: Duration = Duration::from_secs(15);

pub struct GameTimePlugin;

impl Plugin for GameTimePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameTime::new(ROUND_LENGTH))
            .add_event::<GameEnded>()
            .add_systems(
                FixedUpdate,
                (time_drain, debug_time, send_game_end_event_and_game_over)
                    .run_if(on_fixed_timer(Duration::from_secs(1))),
            )
            .add_systems(OnExit(AppState::Playing), end_game);
    }
}
