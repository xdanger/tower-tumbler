use bevy::prelude::*;

#[derive(Resource)]
pub struct TiltInput {
    // Raw device data
    pub beta: f32,  // Front-to-back tilt (-180 to 180 degrees)
    pub gamma: f32, // Left-to-right tilt (-90 to 90 degrees)
    pub alpha: f32, // Compass heading (0 to 360 degrees)

    // Processed data
    pub filtered_beta: f32,
    pub filtered_gamma: f32,

    // Calibration settings
    pub zero_beta: f32,
    pub zero_gamma: f32,
    pub sensitivity: f32,
    pub dead_zone: f32,

    // Control settings
    pub enabled: bool,
    pub input_source: InputSource,

    // Smoothing filter state
    pub ema_alpha: f32,
    pub last_update_time: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputSource {
    Device,
    Keyboard,
    Virtual,
}

impl Default for TiltInput {
    fn default() -> Self {
        Self {
            beta: 0.0,
            gamma: 0.0,
            alpha: 0.0,
            filtered_beta: 0.0,
            filtered_gamma: 0.0,
            zero_beta: 0.0,
            zero_gamma: 0.0,
            sensitivity: 1.0,
            dead_zone: 2.0, // 2 degrees dead zone
            enabled: false,
            input_source: InputSource::Device,
            ema_alpha: 0.3, // Exponential moving average coefficient
            last_update_time: 0.0,
        }
    }
}

impl TiltInput {
    /// Update tilt data with filtering and calibration
    pub fn update_orientation(&mut self, alpha: f32, beta: f32, gamma: f32, timestamp: f64) {
        // Update raw data
        self.alpha = alpha;
        self.beta = beta;
        self.gamma = gamma;
        self.last_update_time = timestamp;

        // Apply calibration (subtract zero points)
        let calibrated_beta = beta - self.zero_beta;
        let calibrated_gamma = gamma - self.zero_gamma;

        // Apply dead zone
        let beta_with_deadzone = self.apply_dead_zone(calibrated_beta);
        let gamma_with_deadzone = self.apply_dead_zone(calibrated_gamma);

        // Apply sensitivity scaling
        let scaled_beta = beta_with_deadzone * self.sensitivity;
        let scaled_gamma = gamma_with_deadzone * self.sensitivity;

        // Apply exponential moving average smoothing
        self.filtered_beta = self.ema_filter(self.filtered_beta, scaled_beta);
        self.filtered_gamma = self.ema_filter(self.filtered_gamma, scaled_gamma);
    }

    /// Apply dead zone to input value
    fn apply_dead_zone(&self, value: f32) -> f32 {
        if value.abs() < self.dead_zone {
            0.0
        } else {
            // Smooth transition from dead zone
            let sign = value.signum();
            let magnitude = value.abs() - self.dead_zone;
            sign * magnitude
        }
    }

    /// Apply exponential moving average filter
    fn ema_filter(&self, previous: f32, current: f32) -> f32 {
        self.ema_alpha * current + (1.0 - self.ema_alpha) * previous
    }

    /// Get normalized tilt values (-1.0 to 1.0)
    pub fn get_normalized_tilt(&self) -> Vec2 {
        if !self.enabled {
            return Vec2::ZERO;
        }

        // Clamp to reasonable tilt range (-45 to 45 degrees) and normalize
        let max_tilt = 45.0;
        let normalized_x = (self.filtered_gamma / max_tilt).clamp(-1.0, 1.0);
        let normalized_y = (self.filtered_beta / max_tilt).clamp(-1.0, 1.0);

        Vec2::new(normalized_x, normalized_y)
    }

    /// Get gravity direction vector for Rapier physics
    pub fn get_gravity_direction(&self) -> Vec2 {
        if !self.enabled {
            return Vec2::new(0.0, -1.0); // Default downward gravity
        }

        let normalized = self.get_normalized_tilt();

        // Map normalized tilt to gravity direction
        // X component: left-right tilt (gamma)
        // Y component: maintain downward bias while allowing forward-back influence
        let gravity_x = normalized.x;
        let gravity_y = -1.0 + normalized.y.abs() * 0.3; // Reduce vertical component slightly

        Vec2::new(gravity_x, gravity_y).normalize_or_zero()
    }

    /// Calibrate current position as zero point
    pub fn calibrate_zero_point(&mut self) {
        self.zero_beta = self.beta;
        self.zero_gamma = self.gamma;
        info!(
            "Calibrated zero point: beta={:.2}°, gamma={:.2}°",
            self.zero_beta, self.zero_gamma
        );
    }

    /// Set sensitivity (0.5 to 2.0)
    pub fn set_sensitivity(&mut self, sensitivity: f32) {
        self.sensitivity = sensitivity.clamp(0.5, 2.0);
    }

    /// Set dead zone in degrees
    pub fn set_dead_zone(&mut self, dead_zone: f32) {
        self.dead_zone = dead_zone.clamp(0.0, 10.0);
    }

    /// Set input source
    pub fn set_input_source(&mut self, source: InputSource) {
        self.input_source = source;
        info!("Input source changed to: {:?}", source);
    }

    /// Get current input status for debugging
    pub fn get_debug_info(&self) -> String {
        format!(
            "TiltInput - Raw: β={:.2}° γ={:.2}° | Filtered: β={:.2}° γ={:.2}° | Normalized: {:.2},{:.2} | Enabled: {} | Source: {:?}",
            self.beta, self.gamma,
            self.filtered_beta, self.filtered_gamma,
            self.get_normalized_tilt().x, self.get_normalized_tilt().y,
            self.enabled, self.input_source
        )
    }
}
