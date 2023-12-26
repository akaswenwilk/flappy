use crate::{score, *};
use bevy::prelude::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui)
            .add_systems(Update, update_score.run_if(in_state(GameState::Started)))
            .add_systems(OnEnter(GameState::Finished), show_final_score);
    }
}

#[derive(Component)]
pub struct UI;

#[derive(Component)]
pub struct UIText;

fn setup_ui(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                transform: Transform::from_xyz(0.0, 0.0, 5.0),
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            UI,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle {
                    text: Text::from_section(
                        "Press SPACEBAR to play!",
                        TextStyle {
                            font_size: 32.0,
                            ..default()
                        },
                    ),
                    ..default()
                },
                UIText,
            ));
        });
}

fn update_score(
    mut ui_style: Query<(&mut Style, &mut BackgroundColor), With<UI>>,
    mut ui_text: Query<&mut Text, With<UIText>>,
    score: Res<score::Score>,
) {
    for (mut style, mut background) in &mut ui_style {
        if style.height == Val::Percent(100.0) {
            style.height = Val::Px(30.0);
            background.0 = Color::BLACK;
        }
    }

    for mut text in &mut ui_text {
        text.sections = vec![TextSection {
            value: format!("Score: {}", score.value / 2),
            style: TextStyle {
                font_size: 32.0,
                color: Color::WHITE,
                ..default()
            },
        }];
    }
}

fn show_final_score(
    mut ui_style: Query<(&mut Style, &mut BackgroundColor), With<UI>>,
    mut ui_text: Query<&mut Text, With<UIText>>,
    score: Res<score::Score>,
) {
    for (mut style, mut background) in &mut ui_style {
        style.height = Val::Percent(100.0);
        background.0 = Color::NONE;
    }
    for mut text in &mut ui_text {
        text.sections = vec![TextSection {
            value: format!(
                "You Lost! Final Score: {}\nPress SPACEBAR to play again!",
                score.value / 2
            ),
            style: TextStyle {
                font_size: 32.0,
                color: Color::WHITE,
                ..default()
            },
        }];
    }
}
