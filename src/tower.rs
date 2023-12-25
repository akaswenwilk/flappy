use crate::{constants::*, GameState};
use bevy::prelude::*;
use bevy::time::Stopwatch;

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_towers)
            .add_systems(Update, spawn_tower.run_if(in_state(GameState::Started)));
    }
}

#[derive(Component)]
pub struct TowerParent {
    asset: Handle<Image>,
    spawn_timer: Timer,
    time_passed: Stopwatch,
    spawn_distance: f32,
}

fn load_towers(mut commands: Commands, asset_server: Res<AssetServer>) {
    let parent = TowerParent {
        asset: asset_server.load("tower.png"),
        spawn_timer: Timer::from_seconds(TOWER_SPAWN_TIME, TimerMode::Repeating),
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
) {
    let (mut parent, mut transform) = parent.single_mut();
    parent.spawn_timer.tick(time.delta());
    parent.time_passed.tick(time.delta());

    transform.translation.x += CAMERA_MOVE_SPEED;

    if parent.spawn_timer.finished() {
        commands.spawn((
            SpriteBundle {
                texture: parent.asset.clone(),
                transform: Transform {
                    translation: Vec3::new(parent.spawn_distance, 0.0, 4.0),
                    scale: Vec3::new(1.0, 1.0, 1.0),
                    ..default()
                },
                ..default()
            },
            Tower,
        ));
        parent.spawn_distance += TOWER_GAP;
    }
}
