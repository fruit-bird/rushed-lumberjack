mod components;
mod events;
mod resources;
mod systems;

use bevy::prelude::*;
use systems::*;

pub use components::{Tree, TreeBundle};
pub use events::TreeChopped;
pub use resources::TreeCount;

pub const SPRITE_TREE_INDEX: usize = 14;
pub const NUMBER_OF_TREES: usize = 25;

pub struct TreePlugin;

impl Plugin for TreePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TreeCount::new(NUMBER_OF_TREES))
            .add_event::<TreeChopped>()
            .add_systems(Startup, spawn_trees)
            .add_systems(Update, (player_collides_with_tree, tree_chopped_sfx));
    }
}
