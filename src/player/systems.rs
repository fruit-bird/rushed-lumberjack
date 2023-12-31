use bevy::prelude::*;

use crate::consts::*;

use super::{
    components::{Health, Player, PlayerBundle},
    PLAYER_VELOCITY, SPRITE_IDX,
};

pub(super) fn spawn_player(
    mut commands: Commands,
    mut atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    let image_handle = asset_server.load("spritesheets/tilemap_packed.png");
    let texture_atlas = TextureAtlas::from_grid(
        image_handle,
        Vec2::new(SPRITE_TILE_WIDTH, SPRITE_TILE_HEIGHT),
        SPRITESHEET_COLS,
        SPRITESHEET_ROWS,
        None,
        None,
    );
    let atlas_handle = atlases.add(texture_atlas);

    commands.spawn(PlayerBundle {
        name: "kiwi".into(),
        health: Health(50),
        sprite: SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(SPRITE_IDX),
            texture_atlas: atlas_handle,
            transform: Transform {
                scale: Vec3::new(
                    SPRITE_RENDER_WIDTH / SPRITE_TILE_WIDTH,
                    SPRITE_RENDER_HEIGHT / SPRITE_TILE_HEIGHT,
                    1.0,
                ),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    });
}

pub(super) fn player_movement(
    mut query: Query<&mut Transform, With<Player>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    if let Ok(mut player_transform) = query.get_single_mut() {
        if input.pressed(KeyCode::D) {
            player_transform.translation.x += time.delta_seconds() * PLAYER_VELOCITY;
        }
        if input.pressed(KeyCode::A) {
            player_transform.translation.x -= time.delta_seconds() * PLAYER_VELOCITY;
        }
        if input.pressed(KeyCode::W) {
            player_transform.translation.y += time.delta_seconds() * PLAYER_VELOCITY;
        }
        if input.pressed(KeyCode::S) {
            player_transform.translation.y -= time.delta_seconds() * PLAYER_VELOCITY;
        }
    }
}
