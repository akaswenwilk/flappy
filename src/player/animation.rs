use bevy::prelude::*;

pub struct Indices {
    pub first: usize,
    pub second: usize,
    pub third: usize,
    pub last: usize,
}

impl Indices {
    pub fn next(&self, index: usize) -> usize {
        match index {
            i if i == self.first => self.second,
            i if i == self.second => self.third,
            i if i == self.third => self.last,
            _ => self.first,
        }
    }
}

#[derive(Component)]
pub struct AnimationTimer(Timer);

impl AnimationTimer {
    pub fn new(duration: f32) -> Self {
        Self(Timer::from_seconds(duration, TimerMode::Repeating))
    }

    pub fn tick(&mut self, time: &Res<Time>) -> bool {
        self.0.tick(time.delta()).finished()
    }

    pub fn just_finished(&self) -> bool {
        self.0.just_finished()
    }
}
