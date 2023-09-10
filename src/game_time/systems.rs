use bevy::prelude::*;
use std::time::Duration;

use crate::game_state::AppState;

use super::{GameEnded, GameTime};

pub(super) fn time_drain(mut game_time: ResMut<GameTime>) {
    // we directly lower time because this system is configured to run once every second
    game_time.secs_remaining = game_time
        .secs_remaining
        .saturating_sub(Duration::from_secs(1));
}

pub(super) fn debug_time(game_time: Res<GameTime>) {
    let secs_remaining = game_time.secs_remaining.as_secs();
    println!("{:2} seconds remaining", secs_remaining);
}

pub(super) fn send_game_end_event_and_game_over(
    mut app_state: ResMut<NextState<AppState>>,
    game_time: Res<GameTime>,
    mut event_writer: EventWriter<GameEnded>,
) {
    if game_time.secs_remaining.as_secs() == 0 {
        event_writer.send_default();
        app_state.set(AppState::GameOver);
    }
}

pub(super) fn end_game(
    mut commands: Commands,
    mut event_reader: EventReader<GameEnded>,
    focused_windows: Query<Entity, With<Window>>,
) {
    if !event_reader.is_empty() {
        event_reader.clear();

        for entity in focused_windows.iter() {
            commands.entity(entity).despawn();
        }
        println!("Time's up!");
    }
}
