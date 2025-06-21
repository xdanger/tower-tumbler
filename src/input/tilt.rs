use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct TiltInput {
    pub beta: f32,  // Device tilt angle in degrees
    pub enabled: bool,
}

impl TiltInput {
    pub fn get_gravity_direction(&self) -> Vec2 {
        if !self.enabled {
            return Vec2::new(0.0, -1.0); // Default downward gravity
        }
        
        // Map beta angle (-180 to 180) to gravity direction
        let angle_rad = self.beta.to_radians();
        Vec2::new(angle_rad.sin(), -angle_rad.cos().abs())
    }
}