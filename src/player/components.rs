use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Player;

#[derive(Component, Default)]
pub struct Health(pub i32); // i32 for now cause u32 causes underflow

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    pub p: Player,
    pub name: Name,
    pub health: Health,
    pub sprite: SpriteSheetBundle,
}
