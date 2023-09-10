use bevy::prelude::*;

use crate::tree::TreeCount;

pub struct AppStatePlugin;

impl Plugin for AppStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>()
            .add_systems(Update, switch_to_game_over)
            .add_systems(
                Update, // change this to load gameover screen later
                execute_shutdown.run_if(in_state(AppState::GameOver)),
            );
    }
}

#[derive(Debug, Default, States, Hash, PartialEq, Eq, Clone)]
pub enum AppState {
    #[default]
    Playing,
    GameOver,
}

pub fn switch_to_game_over(mut app_state: ResMut<NextState<AppState>>, tree_count: Res<TreeCount>) {
    if tree_count.is_zero() {
        println!("Game Over");
        app_state.set(AppState::GameOver);
    }
}

pub fn execute_shutdown(mut commands: Commands, focused_windows: Query<Entity, With<Window>>) {
    for window in focused_windows.iter() {
        commands.entity(window).despawn();
    }
}
