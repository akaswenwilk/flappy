use crate::constants::*;
use crate::player::animation;
use bevy::prelude::*;
use bevy::time::Stopwatch;
use bevy::utils::Duration;

#[derive(Component)]
pub struct Player {
    pub indices: animation::Indices,
    pub timer: animation::AnimationTimer,
    pub flying: bool,
    pub falling: bool,
    pub falling_stopwatch: Stopwatch,
    pub downward_acceleration: f32,
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
            downward_acceleration: GRAVITY,
        }
    }
}

impl Player {
    pub fn initiate_jump(&mut self) {
        self.flying = true;
        self.falling_stopwatch.reset();
        self.downward_acceleration = FLAP_FORCE * GRAVITY;
    }

    pub fn fall(&mut self, passed_time: Duration) -> f32 {
        if !self.falling {
            return 0.0;
        }

        self.falling_stopwatch.tick(passed_time);

        if self.downward_acceleration < GRAVITY {
            self.downward_acceleration += GRAVITY * self.falling_stopwatch.elapsed_secs().powf(2.0);
        };

        self.downward_acceleration * self.falling_stopwatch.elapsed_secs().powf(2.0)
    }

    pub fn rotate(&mut self, fall_amount: f32) -> f32 {
        fall_amount * -0.003
    }
}
