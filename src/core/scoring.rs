use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct ScoreSystem {
    pub perfect_stack_threshold: f32,
    pub perfect_stack_bonus: u32,
    pub base_points: u32,
}

impl ScoreSystem {
    pub fn new() -> Self {
        Self {
            perfect_stack_threshold: 3.0, // pixels
            perfect_stack_bonus: 2,
            base_points: 1,
        }
    }
    
    pub fn calculate_score(&self, deviation: f32) -> u32 {
        if deviation <= self.perfect_stack_threshold {
            self.base_points + self.perfect_stack_bonus
        } else {
            self.base_points
        }
    }
}