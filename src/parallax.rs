use crate::{constants::*, *};
use bevy::prelude::*;
use bevy_parallax::{
    CreateParallaxEvent, LayerData, LayerRepeat, LayerSpeed, ParallaxCameraComponent,
    ParallaxMoveEvent, ParallaxPlugin, ParallaxSystems, RepeatStrategy,
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ParallaxPlugin)
            .add_systems(Startup, initialize_camera_system)
            .add_systems(
                Update,
                move_camera_system
                    .before(ParallaxSystems)
                    .run_if(in_state(GameState::Started)),
            )
            .insert_resource(ClearColor(Color::rgb_u8(42, 0, 63)));
    }
}

// Put a ParallaxCameraComponent on the camera used for parallax
pub fn initialize_camera_system(
    mut commands: Commands,
    mut create_parallax: EventWriter<CreateParallaxEvent>,
) {
    let camera = commands
        .spawn((
            Camera2dBundle::default(),
            ParallaxCameraComponent::default(),
        ))
        .id();

    create_parallax.send(CreateParallaxEvent {
        layers_data: vec![
            layer_data(1.0, "background.png", 0.0),
            layer_data(0.7, "midground.png", 1.0),
            layer_data(0.5, "foreground.png", 2.0),
        ],
        camera,
    })
}

fn layer_data(speed: f32, path: &str, z: f32) -> LayerData {
    LayerData {
        speed: LayerSpeed::Horizontal(speed),
        path: path.to_string(),
        z,
        repeat: LayerRepeat::horizontally(RepeatStrategy::Same),
        tile_size: Vec2::new(TILE_WIDTH, TILE_HEIGHT),
        cols: 1,
        rows: 1,
        scale: 3.5,
        ..default()
    }
}

// Send a ParallaxMoveEvent with the desired camera movement speed
pub fn move_camera_system(
    mut move_event_writer: EventWriter<ParallaxMoveEvent>,
    camera_query: Query<Entity, With<ParallaxCameraComponent>>,
) {
    let camera = camera_query.get_single().unwrap();

    move_event_writer.send(ParallaxMoveEvent {
        camera_move_speed: Vec2::new(CAMERA_MOVE_SPEED, 0.0),
        camera,
    });
}
