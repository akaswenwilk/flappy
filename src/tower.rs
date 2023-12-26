use crate::{constants::*, GameState};
use bevy::{prelude::*, sprite::Anchor, time::Stopwatch};

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_towers).add_systems(
            Update,
            (spawn_tower, despawn_tower).run_if(in_state(GameState::Started)),
        );
    }
}

#[derive(Resource)]
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
    commands.insert_resource(parent);
}

#[derive(Component)]
pub struct Tower;

fn spawn_tower(
    time: Res<Time>,
    mut commands: Commands,
    mut parent: ResMut<TowerParent>,
    towers: Query<(Entity, &Tower)>,
) {
    parent.time_passed.tick(time.delta());

    let mut towers_count = 0;
    for _ in &towers {
        towers_count += 1;
    }
    if towers_count >= MAX_TOWER_SPAWN {
        return;
    }

    let base = -WINDOW_HEIGHT / 2.;

    commands.spawn((
        Name::new(format!("Tower {:?}", parent.spawn_distance)),
        tower_bundle(&parent, base, false),
        Tower,
    ));

    commands.spawn((
        Name::new(format!("Tower {:?}", parent.spawn_distance)),
        tower_bundle(&parent, base + WINDOW_HEIGHT + VERTICAL_TOWER_GAP, true),
        Tower,
    ));
    parent.spawn_distance += TOWER_GAP;
}

fn tower_bundle(parent: &ResMut<TowerParent>, y_offset: f32, flip_y: bool) -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite {
            flip_y,
            custom_size: Some(Vec2::new(TOWER_WIDTH, WINDOW_HEIGHT)),
            anchor: Anchor::CenterLeft,
            ..default()
        },
        texture: parent.asset.clone(),
        transform: Transform {
            translation: Vec3::new(parent.spawn_distance, y_offset, 4.0),
            scale: Vec3::new(1., 1., 1.),
            ..default()
        },
        ..default()
    }
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
