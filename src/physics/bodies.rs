use crate::core::{Block, Ground};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn create_ground(commands: &mut Commands, position: Vec2, size: Vec2) -> Entity {
    commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(0.3, 0.3, 0.3),
                    custom_size: Some(size),
                    ..default()
                },
                transform: Transform::from_translation(position.extend(0.0)),
                ..default()
            },
            RigidBody::Fixed,
            Collider::cuboid(size.x / 2.0, size.y / 2.0),
            Ground,
        ))
        .id()
}

pub fn create_block(commands: &mut Commands, position: Vec2, size: Vec2) -> Entity {
    commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(0.8, 0.4, 0.2),
                    custom_size: Some(size),
                    ..default()
                },
                transform: Transform::from_translation(position.extend(0.0)),
                ..default()
            },
            RigidBody::Dynamic,
            Collider::cuboid(size.x / 2.0, size.y / 2.0),
            Restitution::coefficient(0.3),
            Friction::coefficient(0.7),
            Block {
                size,
                settled: false,
            },
        ))
        .id()
}
