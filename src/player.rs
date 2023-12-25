mod animation;
mod player;

use crate::constants::*;
use bevy::{asset::LoadedFolder, prelude::*};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<PlayerState>()
            .add_systems(OnEnter(PlayerState::Loading), load_player_assets)
            .add_systems(Update, check_loaded.run_if(in_state(PlayerState::Loading)))
            .add_systems(OnEnter(PlayerState::Ready), generate_player)
            .add_systems(Update, (move_player, animate_player));
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum PlayerState {
    #[default]
    Loading,
    Ready,
}

#[derive(Resource)]
pub struct PlayerSprites(Handle<LoadedFolder>);

fn load_player_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player_folder = asset_server.load_folder("player");

    commands.insert_resource(PlayerSprites(player_folder));
}

fn check_loaded(
    sprites: Res<PlayerSprites>,
    mut events: EventReader<AssetEvent<LoadedFolder>>,
    mut next_state: ResMut<NextState<PlayerState>>,
) {
    for event in events.read() {
        if event.is_loaded_with_dependencies(&sprites.0) {
            next_state.set(PlayerState::Ready);
        }
    }
}

fn generate_player(
    mut commands: Commands,
    sprites: Res<PlayerSprites>,
    loaded_folders: Res<Assets<LoadedFolder>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
) {
    let loaded_folder = loaded_folders.get(&sprites.0).unwrap();

    let mut texture_atlas_builder = TextureAtlasBuilder::default();

    for handle in loaded_folder.handles.iter() {
        let id = handle.id().typed_unchecked::<Image>();
        let Some(texture) = textures.get(id) else {
            warn!(
                "{:?} did not resolve to an `Image` asset.",
                handle.path().unwrap()
            );
            continue;
        };

        texture_atlas_builder.add_texture(id, texture);
    }

    let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
    let atlas_handle = texture_atlases.add(texture_atlas.clone());
    let animation_indices = animation::Indices {
        first: 0,
        second: 2,
        third: 1,
        last: 3,
    };
    let timer = animation::AnimationTimer::new(ANIMATION_TIMER);

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: atlas_handle,
            sprite: TextureAtlasSprite {
                index: animation_indices.last,
                custom_size: Some(Vec2::new(SPRITE_SIZE, SPRITE_SIZE)),
                ..default()
            },
            transform: Transform::from_xyz(-400.0, 0.0, 3.0),
            ..default()
        },
        player::Player {
            indices: animation_indices,
            timer,
            ..default()
        },
    ));
}

fn move_player(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut player::Player)>,
) {
    for (mut transform, mut player) in query.iter_mut() {
        transform.translation.x += CAMERA_MOVE_SPEED;

        if input.just_pressed(KeyCode::Space) {
            player.flying = true;
            player.falling = true;
            player.falling_stopwatch.reset();
            player.downward_acceleration = -5.0 * GRAVITY;
        }
        if player.falling {
            player.falling_stopwatch.tick(time.delta());
            let fall =
                player.downward_acceleration * player.falling_stopwatch.elapsed_secs().powf(2.0);
            transform.translation.y -= fall;
            if player.downward_acceleration < GRAVITY {
                player.downward_acceleration += GRAVITY * player.falling_stopwatch.elapsed_secs().powf(2.0);
            }
        }
    }
}

fn animate_player(
    time: Res<Time>,
    mut query: Query<(&mut player::Player, &mut TextureAtlasSprite)>,
) {
    for (mut player, mut sprite) in &mut query {
        if player.flying {
            player.timer.tick(&time);
            if player.timer.just_finished() {
                sprite.index = player.indices.next(sprite.index);
                if sprite.index == player.indices.last {
                    player.flying = false;
                }
            }
        }
    }
}
