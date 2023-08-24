use bevy::prelude::*;

use super::NUMBER_OF_TREES;

#[derive(Debug, Resource)]
pub struct TreeCount(pub usize);

impl TreeCount {
    pub const fn new(count: usize) -> Self {
        Self(count)
    }

    #[inline]
    pub const fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

impl Default for TreeCount {
    fn default() -> Self {
        Self::new(NUMBER_OF_TREES)
    }
}
