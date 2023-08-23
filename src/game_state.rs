use bevy::prelude::*;

use crate::{
    player::{player_is_dead, Player},
    tree::{no_more_trees, TreeCount},
};

#[derive(Debug, Default, States, Hash, PartialEq, Eq, Clone)]
pub enum AppState {
    #[default]
    Playing,
    GameOver,
}

pub fn game_over(
    mut app_state: ResMut<NextState<AppState>>,
    player_query: Query<(), With<Player>>,
    tree_count: Res<TreeCount>,
) {
    if player_is_dead(player_query) | no_more_trees(tree_count) {
        info!("Game Over");
        app_state.set(AppState::GameOver);
    }
}
