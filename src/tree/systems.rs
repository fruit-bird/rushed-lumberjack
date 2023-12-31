use bevy::{prelude::*, sprite::collide_aabb, window::PrimaryWindow};

use super::{
    components::{Tree, TreeBundle},
    events::TreeChopped,
    resources::TreeCount,
    NUMBER_OF_TREES, SPRITE_TREE_INDEX,
};
use crate::{consts::*, player::Player};

pub(super) fn spawn_trees(
    mut commands: Commands,
    mut atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
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

    let window = match window_query.get_single() {
        Ok(w) => w,
        Err(_) => return,
    };

    for _ in 0..NUMBER_OF_TREES {
        let pos_neg_x = if rand::random::<bool>() { 1.0 } else { -1.0 };
        let pos_neg_y = if rand::random::<bool>() { 1.0 } else { -1.0 };
        let rand_x = rand::random::<f32>() * window.width() / 2.0 * pos_neg_x - SPRITE_TILE_WIDTH;
        let rand_y = rand::random::<f32>() * window.height() / 2.0 * pos_neg_y - SPRITE_TILE_HEIGHT;

        commands.spawn(TreeBundle {
            t: Tree,
            sprite: SpriteSheetBundle {
                sprite: TextureAtlasSprite::new(SPRITE_TREE_INDEX),
                texture_atlas: atlas_handle.clone(),
                transform: Transform {
                    scale: Vec3::new(
                        SPRITE_RENDER_WIDTH / SPRITE_TILE_WIDTH,
                        SPRITE_RENDER_HEIGHT / SPRITE_TILE_HEIGHT,
                        1.0,
                    ),
                    translation: Vec3::new(rand_x, rand_y, 1.0),
                    ..Default::default()
                },
                ..Default::default()
            },
        });
    }
}

pub(super) fn player_collides_with_tree(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    trees_query: Query<(Entity, &Transform), With<Tree>>,
    mut tree_count: ResMut<TreeCount>,
    mut event_writer: EventWriter<TreeChopped>,
) {
    if let Ok(player_pos) = player_query.get_single() {
        for (entity, tree_pos) in trees_query.iter() {
            if collide_aabb::collide(
                player_pos.translation,
                SPRITE_SIZE * 0.5, // allowing some overlap
                tree_pos.translation,
                SPRITE_SIZE * 0.5,
            )
            .is_some()
            {
                event_writer.send_default();
                commands.entity(entity).despawn();
                tree_count.0 -= 1;
            }
        }
    }
}

pub(super) fn play_tree_chopped_sfx(
    mut commands: Commands,
    event_reader: EventReader<TreeChopped>,
    asset_server: Res<AssetServer>,
) {
    if !event_reader.is_empty() {
        commands.spawn(AudioBundle {
            source: asset_server.load("audio/biden-skill-issue.ogg"),
            ..Default::default()
        });
    }
}
