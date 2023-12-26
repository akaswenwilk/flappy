mod constants;
mod parallax;
mod player;
mod tower;
mod collision;

use crate::constants::*;
use bevy::input::common_conditions::input_toggle_active;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
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
        .add_plugins(collision::CollisionPlugin)
        .add_systems(Update, start_game.run_if(in_state(GameState::Stopped)))
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
