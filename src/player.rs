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

#[derive(Component)]
pub struct Player;

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
    let animation_indices = AnimationIndices { first: 0, second: 2, third: 1, last: 3 };

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: atlas_handle,
            sprite: TextureAtlasSprite {
                index: animation_indices.first,
                custom_size: Some(Vec2::new(SPRITE_SIZE, SPRITE_SIZE)),
                ..default()
            },
            transform: Transform::from_xyz(-400.0, 0.0, 3.0),
            ..default()
        },
        Player,
        animation_indices,
        AnimationTimer(Timer::from_seconds(ANIMATION_TIMER, TimerMode::Repeating)),
    ));
}

fn move_player(mut query: Query<&mut Transform, With<Player>>) {
    for mut transform in query.iter_mut() {
        transform.translation.x += CAMERA_MOVE_SPEED;
    }
}

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    second: usize,
    third: usize,
    last: usize,
}

impl AnimationIndices {
    fn next(&self, index: usize) -> usize {
        match index {
            i if i == self.first => self.second,
            i if i == self.second => self.third,
            i if i == self.third => self.last,
            _ => self.first,
        }
    }
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_player(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut Transform,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (indices, mut transform, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = indices.next(sprite.index);
            let mut move_amount = 5.0;
            if sprite.index == indices.last {
                move_amount = move_amount * -3.0;
            }
            transform.translation.y += move_amount;
        }
    }
}
