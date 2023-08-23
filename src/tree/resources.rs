use bevy::prelude::*;

use super::NUMBER_OF_TREES;

#[derive(Debug, Resource)]
pub struct TreeCount(pub usize);

impl TreeCount {
    pub fn new(count: usize) -> Self {
        Self(count)
    }
}

impl Default for TreeCount {
    fn default() -> Self {
        Self::new(NUMBER_OF_TREES)
    }
}
