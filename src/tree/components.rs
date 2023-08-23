use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Tree;

#[derive(Bundle)]
pub struct TreeBundle {
    pub t: Tree,
    pub sprite: SpriteSheetBundle,
}
