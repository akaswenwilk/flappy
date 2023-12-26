use crate::{player, *};
use bevy::prelude::*;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score { value: 0 })
            .add_systems(Update, update_score.run_if(in_state(GameState::Started)));
    }
}

#[derive(Resource)]
pub struct Score {
    pub value: u32,
}

fn update_score(
    player: Query<&GlobalTransform, With<player::player::Player>>,
    mut towers: Query<(&GlobalTransform, &mut tower::Tower)>,
    mut score: ResMut<Score>,
) {
    for player in &mut player.iter() {
        for (tower_transform, mut tower) in &mut towers {
            if tower.passed {
                continue;
            }

            if player.translation().x > tower_transform.translation().x {
                score.value += 1;
                tower.passed = true;
            }
        }
    }
}
