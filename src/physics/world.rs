use crate::input::TiltInput;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            // .add_plugins(RapierDebugRenderPlugin::default())  // Disabled for WASM compatibility
            .add_systems(Startup, setup_physics_world)
            .add_systems(Update, update_gravity);
    }
}

fn setup_physics_world(mut commands: Commands) {
    // Initialize physics world with default gravity
    commands.insert_resource(GravityManager::default());
    info!("Physics world initialized");
}

#[derive(Resource, Default)]
pub struct GravityManager {
    pub base_gravity: f32,
    pub last_gravity_update: f64,
    pub update_interval: f64,
}

impl GravityManager {
    pub fn new() -> Self {
        Self {
            base_gravity: 980.0, // 9.8 m/s² in pixels/s² (100 pixels per meter)
            last_gravity_update: 0.0,
            update_interval: 16.0, // Update every 16ms (~60Hz)
        }
    }
}

fn update_gravity(
    tilt_input: Res<TiltInput>,
    mut gravity_manager: ResMut<GravityManager>,
    mut rapier_config: ResMut<RapierConfiguration>,
    time: Res<Time>,
) {
    let current_time = time.elapsed_seconds_f64() * 1000.0; // Convert to milliseconds

    // Throttle gravity updates to maintain performance
    if current_time - gravity_manager.last_gravity_update < gravity_manager.update_interval {
        return;
    }
    gravity_manager.last_gravity_update = current_time;

    // Get gravity direction from tilt input
    let gravity_direction = tilt_input.get_gravity_direction();

    // Apply base gravity magnitude to the direction
    let gravity_vector = gravity_direction * gravity_manager.base_gravity;

    // Update Rapier gravity
    rapier_config.gravity = gravity_vector;

    // Debug log gravity changes (throttled)
    if tilt_input.enabled {
        debug!(
            "Gravity updated: direction=({:.3}, {:.3}), magnitude={:.1}",
            gravity_direction.x, gravity_direction.y, gravity_manager.base_gravity
        );
    }
}
