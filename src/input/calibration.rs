use super::{InputSource, TiltInput};
use bevy::prelude::*;

/// System for handling tilt input calibration
pub fn handle_calibration_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut tilt_input: ResMut<TiltInput>,
) {
    // Press 'C' to calibrate zero point
    if keyboard_input.just_pressed(KeyCode::KeyC) {
        tilt_input.calibrate_zero_point();
    }

    // Press number keys to adjust sensitivity
    if keyboard_input.just_pressed(KeyCode::Digit1) {
        tilt_input.set_sensitivity(0.5);
        info!("Sensitivity set to 0.5x");
    }
    if keyboard_input.just_pressed(KeyCode::Digit2) {
        tilt_input.set_sensitivity(1.0);
        info!("Sensitivity set to 1.0x");
    }
    if keyboard_input.just_pressed(KeyCode::Digit3) {
        tilt_input.set_sensitivity(1.5);
        info!("Sensitivity set to 1.5x");
    }
    if keyboard_input.just_pressed(KeyCode::Digit4) {
        tilt_input.set_sensitivity(2.0);
        info!("Sensitivity set to 2.0x");
    }

    // Press 'D' to toggle input source between Device/Keyboard/Virtual
    if keyboard_input.just_pressed(KeyCode::KeyD) {
        let new_source = match tilt_input.input_source {
            InputSource::Device => InputSource::Keyboard,
            InputSource::Keyboard => InputSource::Virtual,
            InputSource::Virtual => InputSource::Device,
        };
        tilt_input.set_input_source(new_source);
    }

    // Adjust dead zone with '+' and '-' keys
    if keyboard_input.just_pressed(KeyCode::Equal) {
        // '+' key
        let new_dead_zone = tilt_input.dead_zone + 0.5;
        tilt_input.set_dead_zone(new_dead_zone);
        info!("Dead zone increased to {:.1}°", tilt_input.dead_zone);
    }
    if keyboard_input.just_pressed(KeyCode::Minus) {
        let new_dead_zone = tilt_input.dead_zone - 0.5;
        tilt_input.set_dead_zone(new_dead_zone);
        info!("Dead zone decreased to {:.1}°", tilt_input.dead_zone);
    }

    // Press 'I' to show current input info
    if keyboard_input.just_pressed(KeyCode::KeyI) {
        info!("{}", tilt_input.get_debug_info());
    }
}

/// System for handling keyboard input as fallback tilt control
pub fn handle_keyboard_tilt_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut tilt_input: ResMut<TiltInput>,
    time: Res<Time>,
) {
    if tilt_input.input_source != InputSource::Keyboard {
        return;
    }

    let current_time = time.elapsed_seconds_f64() * 1000.0;
    let mut beta = tilt_input.beta;
    let mut gamma = tilt_input.gamma;

    // Simulate device tilt with keyboard
    let tilt_speed = 30.0; // degrees per second
    let delta_time = time.delta_seconds();
    let tilt_increment = tilt_speed * delta_time;

    // Use arrow keys to simulate tilt
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        gamma -= tilt_increment;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        gamma += tilt_increment;
    }
    if keyboard_input.pressed(KeyCode::ArrowUp) {
        beta -= tilt_increment;
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) {
        beta += tilt_increment;
    }

    // Clamp to reasonable ranges
    beta = beta.clamp(-45.0, 45.0);
    gamma = gamma.clamp(-45.0, 45.0);

    // Update tilt input with simulated data
    tilt_input.update_orientation(0.0, beta, gamma, current_time);
    tilt_input.enabled = true;
}

/// System for virtual button input (for touch devices without tilt)
pub fn handle_virtual_tilt_input(mut tilt_input: ResMut<TiltInput>, _time: Res<Time>) {
    if tilt_input.input_source != InputSource::Virtual {
        return;
    }

    // Virtual input will be implemented when UI buttons are added
    // For now, just ensure the input is enabled
    tilt_input.enabled = true;
}
