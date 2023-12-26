use crate::constants::*;
use crate::player::animation;
use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub indices: animation::Indices,
    pub timer: animation::AnimationTimer,
    pub flying: bool,
    pub falling: bool,
    pub velocity: f32,
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
            flying: false,
            falling: false,
            velocity: 0.0,
        }
    }
}

impl Player {
    pub fn initiate_jump(&mut self) {
        self.flying = true;
        self.velocity = FLAP_VELOCITY;
    }

    pub fn fall(&mut self, passed_time: f32) -> f32 {
        self.velocity += -GRAVITY * passed_time;

        self.velocity * passed_time
    }

    pub fn rotate(&mut self, fall_amount: f32) -> f32 {
        fall_amount * 0.003
    }

    pub fn reset(&mut self) {
        self.velocity = 0.0;
    }
}
