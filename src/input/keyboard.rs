use bevy::prelude::*;
use super::TiltInput;

pub fn handle_keyboard_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut tilt_input: ResMut<TiltInput>,
) {
    let beta;
    
    if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
        beta = -10.0; // Simulate left tilt
    } else if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
        beta = 10.0; // Simulate right tilt
    } else {
        beta = 0.0; // No tilt
    }
    
    tilt_input.beta = beta;
}