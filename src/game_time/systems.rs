use bevy::prelude::*;
use std::time::Duration;

use super::{GameEnded, GameTime};

pub(super) fn time_drain(mut game_time: ResMut<GameTime>) {
    // we directly lower time because this system is configured to run once every second
    game_time.secs_remaining -= Duration::from_secs(1);
}

pub(super) fn debug_time(game_time: Res<GameTime>) {
    let secs_remaining = game_time.secs_remaining.as_secs();
    println!("{:2} seconds remaining", secs_remaining);
}

pub(super) fn send_game_end_event(
    game_time: Res<GameTime>,
    mut event_writer: EventWriter<GameEnded>,
) {
    if game_time.secs_remaining.as_secs() == 0 {
        event_writer.send_default()
    }
}

pub(super) fn end_game(mut event_reader: EventReader<GameEnded>) {
    if !event_reader.is_empty() {
        event_reader.clear();
        todo!("Gracefully end the game");
    }
}
