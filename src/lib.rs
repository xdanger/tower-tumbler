use bevy::prelude::*;

pub mod core;
pub mod input;
pub mod physics;
pub mod ui;

pub use self::core::*;

pub struct TowerTumblerPlugin;

impl Plugin for TowerTumblerPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .init_resource::<input::TiltInput>()
            .add_plugins(physics::PhysicsPlugin)
            .add_systems(Startup, setup_game)
            .add_systems(
                Update,
                (
                    handle_game_input,
                    update_game_state,
                    input::handle_keyboard_input,
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(States, Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub enum GameState {
    #[default]
    MainMenu,
    Playing,
    GameOver,
}

#[derive(Component)]
pub struct GameCamera;

#[derive(Resource, Default)]
pub struct GameScore {
    pub current: u32,
    pub best: u32,
}

fn setup_game(mut commands: Commands) {
    // Initialize game camera
    commands.spawn((Camera2dBundle::default(), GameCamera));

    // Initialize game score
    commands.insert_resource(GameScore::default());

    // Trigger WASM loaded event
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::prelude::*;

        #[wasm_bindgen]
        extern "C" {
            #[wasm_bindgen(js_namespace = window)]
            fn dispatchEvent(event: &web_sys::Event);
        }

        let window = web_sys::window().unwrap();
        let event = web_sys::CustomEvent::new("wasmLoaded").unwrap();
        window.dispatch_event(&event).unwrap();
    }

    info!("Tower Tumbler initialized successfully");
}

fn handle_game_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::MainMenu);
    }
}

fn update_game_state() {
    // Game state update logic will be implemented here
}
