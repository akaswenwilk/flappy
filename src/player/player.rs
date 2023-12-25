use crate::player::animation;
use bevy::prelude::*;
use bevy::time::Stopwatch;

#[derive(Component)]
pub struct Player {
    pub indices: animation::Indices,
    pub timer: animation::AnimationTimer,
    pub flying: bool,
    pub falling: bool,
    pub falling_stopwatch: Stopwatch,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            indices: animation::Indices {
                first: 0,
                second: 0,
                third: 0,
                last: 0,
            },
            timer: animation::AnimationTimer::new(0.0),
            falling_stopwatch: Stopwatch::new(),
            flying: false,
            falling: false,
        }
    }
}
