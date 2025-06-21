use bevy::prelude::*;

#[derive(Component)]
pub struct Block {
    pub size: Vec2,
    pub settled: bool,
}

#[derive(Component)]
pub struct Ground;

#[derive(Component)]
pub struct Tower {
    pub height: f32,
    pub blocks: Vec<Entity>,
}

impl Default for Block {
    fn default() -> Self {
        Self {
            size: Vec2::new(60.0, 20.0),
            settled: false,
        }
    }
}