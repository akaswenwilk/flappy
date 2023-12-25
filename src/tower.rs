use crate::{constants::*, GameState};
use bevy::{prelude::*, sprite::Anchor, time::Stopwatch};

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_towers)
            .add_systems(Update, spawn_tower.run_if(in_state(GameState::Started)))
            .add_systems(Update, despawn_tower);
    }
}

#[derive(Component)]
pub struct TowerParent {
    asset: Handle<Image>,
    time_passed: Stopwatch,
    spawn_distance: f32,
}

fn load_towers(mut commands: Commands, asset_server: Res<AssetServer>) {
    let parent = TowerParent {
        asset: asset_server.load("tower.png"),
        time_passed: Stopwatch::new(),
        spawn_distance: WINDOW_WIDTH,
    };
    commands.spawn((Name::new("tower_parent"), SpatialBundle::default(), parent));
}

#[derive(Component)]
pub struct Tower;

fn spawn_tower(
    time: Res<Time>,
    mut commands: Commands,
    mut parent: Query<(&mut TowerParent, &mut Transform)>,
    towers: Query<(Entity, &Tower)>,
) {
    let (mut parent, mut transform) = parent.single_mut();
    parent.time_passed.tick(time.delta());

    transform.translation.x += CAMERA_MOVE_SPEED;

    let mut towers_count = 0;
    for _ in &towers {
        towers_count += 1;
    }
    if towers_count >= MAX_TOWER_SPAWN {
        return;
    }

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(TOWER_WIDTH, WINDOW_HEIGHT)),
                anchor: Anchor::BottomLeft,
                ..default()
            },
            texture: parent.asset.clone(),
            transform: Transform {
                translation: Vec3::new(parent.spawn_distance, -WINDOW_HEIGHT / 2.0, 4.0),
                scale: Vec3::new(1.0, 1.0, 1.0),
                ..default()
            },
            ..default()
        },
        Tower,
    ));
    parent.spawn_distance += TOWER_GAP;
}

fn despawn_tower(
    mut commands: Commands,
    mut towers: Query<(Entity, &Transform), With<Tower>>,
    cam: Query<&GlobalTransform, With<Camera>>,
) {
    let cam = cam.single();
    let cam_x = cam.translation().x;

    for (entity, transform) in &mut towers {
        if transform.translation.x < cam_x - WINDOW_WIDTH {
            commands.entity(entity).despawn();
        }
    }
}
