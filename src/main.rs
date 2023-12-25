mod constants;
mod parallax;
mod player;

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
        .run();
}
