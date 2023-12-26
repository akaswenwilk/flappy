mod collision;
mod constants;
mod parallax;
mod player;
mod score;
mod tower;
mod ui;

use crate::constants::*;
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
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
        .add_plugins(ui::UIPlugin)
        .add_plugins(score::ScorePlugin)
        .add_systems(Update, start_game.run_if(in_state(GameState::Stopped)))
        .add_systems(Update, restart_game.run_if(in_state(GameState::Finished)))
        .run();
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum GameState {
    #[default]
    Stopped,
    Started,
    Finished,
}

fn start_game(mut state: ResMut<NextState<GameState>>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        state.set(GameState::Started);
    }
}

fn restart_game(
    mut commands: Commands,
    mut state: ResMut<NextState<GameState>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut score: ResMut<score::Score>,
    towers: Query<Entity, With<tower::Tower>>,
    mut player: Query<(
        &mut Transform,
        &mut TextureAtlasSprite,
        &mut player::player::Player,
    )>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        state.set(GameState::Started);
        score.value = 0;

        for tower in &towers {
            commands.entity(tower).despawn_recursive();
        }

        for (mut transform, mut sprite, mut player) in &mut player {
            player.reset();
            sprite.color = Color::WHITE;
            transform.translation.y = 0.;
        }

        state.set(GameState::Started);
    }
}
