use crate::{constants::*, *};
use bevy::prelude::*;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            collision_detection.run_if(in_state(GameState::Started)),
        );
    }
}

fn collision_detection(
    mut state: ResMut<NextState<GameState>>,
    mut players: Query<(&GlobalTransform, &mut TextureAtlasSprite), With<player::player::Player>>,
    mut towers: Query<(&GlobalTransform, &mut Sprite), With<tower::Tower>>,
) {
    for (player_transform, mut player_sprite) in &mut players {
        let player_coordinates = player_transform.translation();
        if player_coordinates.y <= -WINDOW_HEIGHT / 2.
            || player_coordinates.y >= (WINDOW_HEIGHT / 2.) - SPRITE_SIZE
        {
            player_sprite.color = Color::RED;
        }

        for (tower_transform, mut tower_sprite) in &mut towers {
            let tower_coordinates = tower_transform.translation();
            if collide_bottom_corner(player_coordinates, tower_coordinates)
                || collide_top_corner(player_coordinates, tower_coordinates)
            {
                player_sprite.color = Color::RED;
                tower_sprite.color = Color::RED;
                break;
            }
        }

        if player_sprite.color == Color::RED {
            state.set(GameState::Finished);
        }
    }
}

fn collide_bottom_corner(player_coordinates: Vec3, tower_coordinates: Vec3) -> bool {
    player_coordinates.x + SPRITE_SIZE >= tower_coordinates.x
        && player_coordinates.x + SPRITE_SIZE <= tower_coordinates.x + TOWER_WIDTH
        && player_coordinates.y >= tower_coordinates.y - WINDOW_HEIGHT / 2.
        && player_coordinates.y
            <= tower_coordinates.y + (WINDOW_HEIGHT / 2.) - TOWER_TOP_COLLISION_OFFSET
}

fn collide_top_corner(player_coordinates: Vec3, tower_coordinates: Vec3) -> bool {
    player_coordinates.x + SPRITE_SIZE >= tower_coordinates.x
        && player_coordinates.x + SPRITE_SIZE <= tower_coordinates.x + TOWER_WIDTH
        && player_coordinates.y + SPRITE_SIZE
            >= tower_coordinates.y - WINDOW_HEIGHT / 2. + TOWER_TOP_COLLISION_OFFSET
        && player_coordinates.y + SPRITE_SIZE <= tower_coordinates.y + WINDOW_HEIGHT / 2.
}
