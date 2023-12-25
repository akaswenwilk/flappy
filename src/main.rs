mod constants;
mod parallax;
mod player;
mod tower;

use crate::constants::*;
use bevy::input::common_conditions::input_toggle_active;
use bevy::{
    prelude::*,
    sprite::{Anchor, MaterialMesh2dBundle},
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    // Define window
    let primary_window = Window {
        title: "Flappy".to_string(),
        resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
        resizable: false,
        ..Default::default()
    };

    App::new()
        .add_state::<GameState>()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(primary_window),
                    ..default()
                })
                // Use nearest filtering so our pixel art renders clear
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),
        )
        .add_plugins(parallax::CameraPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(tower::TowerPlugin)
        .add_systems(Update, start_game)
        .add_systems(Update, draw_rectangles)
        .run();
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum GameState {
    #[default]
    Stopped,
    Started,
}

fn start_game(mut state: ResMut<NextState<GameState>>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        state.set(GameState::Started);
    }
}

#[derive(Component)]
pub struct Rectangle;

#[derive(Component)]
pub struct DrawRectangles;

fn draw_rectangles(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
    players: Query<(Entity, &GlobalTransform), (With<player::player::Player>, Without<Rectangle>)>,
    towers: Query<(Entity, &GlobalTransform), (With<tower::Tower>, Without<Rectangle>)>,
) {
    for (entity, transform) in &players {
        commands.entity(entity).insert(Rectangle);
        let mut coordinates = transform.translation();
        coordinates.z += 100.;
        coordinates.x += PLAYER_OFFSET;
        commands.spawn((
            Name::new("player_rectangle"),
            SpriteBundle {
                sprite: Sprite {
                    anchor: Anchor::BottomLeft,
                    color: Color::LIME_GREEN,
                    custom_size: Some(Vec2::new(SPRITE_SIZE, SPRITE_SIZE)),
                    ..default()
                },
                transform: Transform::from_translation(coordinates),
                ..default()
            },
            Rectangle,
            DrawRectangles,
            player::player::Player::default(),
        ));
    }
    for (entity, transform) in &towers {
        commands.entity(entity).insert(Rectangle);
        let mut coordinates = transform.translation();
        coordinates.z -= 1.;
        coordinates.x += PLAYER_OFFSET;
        commands.spawn((
            Name::new("tower_rectangle"),
            MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::Quad::new(Vec2::new(10., 10.)).into())
                    .into(),
                material: materials.add(ColorMaterial::from(Color::LIME_GREEN)),
                transform: Transform::from_translation(coordinates),
                ..default()
            },
            Rectangle,
        ));
    }
}
