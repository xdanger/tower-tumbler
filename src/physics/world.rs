use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            // .add_plugins(RapierDebugRenderPlugin::default())  // Disabled for WASM compatibility
            .add_systems(Startup, setup_physics_world);
        // .add_systems(Update, update_gravity);  // Temporarily disabled until gravity system is implemented
    }
}

fn setup_physics_world(_commands: Commands) {
    // Physics world will be automatically created by Rapier
    info!("Physics world initialized");
}
