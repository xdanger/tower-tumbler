use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceOrientationData {
    pub alpha: f32, // Z-axis rotation
    pub beta: f32,  // X-axis rotation (front-to-back tilt)
    pub gamma: f32, // Y-axis rotation (left-to-right tilt)
    pub timestamp: f64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PermissionStatus {
    pub granted: bool,
    pub requested: bool,
    pub available: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum BridgeEvent {
    DeviceOrientation(DeviceOrientationData),
    PermissionStatusChanged(PermissionStatus),
    GameStateRequest { state: String },
    GameStateResponse { state: String, data: String },
}

// Global bridge instance for communication between JS and Rust
lazy_static::lazy_static! {
    static ref BRIDGE: Arc<Mutex<EventBridge>> = Arc::new(Mutex::new(EventBridge::new()));
}

pub struct EventBridge {
    event_queue: VecDeque<BridgeEvent>,
    max_queue_size: usize,
    permission_status: PermissionStatus,
}

impl EventBridge {
    fn new() -> Self {
        Self {
            event_queue: VecDeque::new(),
            max_queue_size: 120, // ~2 seconds at 60Hz
            permission_status: PermissionStatus {
                granted: false,
                requested: false,
                available: false,
            },
        }
    }

    pub fn push_event(&mut self, event: BridgeEvent) {
        if self.event_queue.len() >= self.max_queue_size {
            self.event_queue.pop_front();
        }
        self.event_queue.push_back(event);
    }

    pub fn pop_event(&mut self) -> Option<BridgeEvent> {
        self.event_queue.pop_front()
    }

    pub fn get_permission_status(&self) -> &PermissionStatus {
        &self.permission_status
    }

    pub fn set_permission_status(&mut self, status: PermissionStatus) {
        self.permission_status = status.clone();
        self.push_event(BridgeEvent::PermissionStatusChanged(status));
    }
}

// WASM-bindgen interface functions for JavaScript interaction
#[wasm_bindgen]
pub fn js_push_device_orientation(alpha: f32, beta: f32, gamma: f32, timestamp: f64) {
    let data = DeviceOrientationData {
        alpha,
        beta,
        gamma,
        timestamp,
    };

    if let Ok(mut bridge) = BRIDGE.lock() {
        bridge.push_event(BridgeEvent::DeviceOrientation(data));
    }
}

#[wasm_bindgen]
pub fn js_set_permission_status(granted: bool, requested: bool, available: bool) {
    let status = PermissionStatus {
        granted,
        requested,
        available,
    };

    if let Ok(mut bridge) = BRIDGE.lock() {
        bridge.set_permission_status(status);
    }
}

#[wasm_bindgen]
pub fn js_request_game_state(state: &str) -> String {
    let event = BridgeEvent::GameStateRequest {
        state: state.to_string(),
    };

    if let Ok(mut bridge) = BRIDGE.lock() {
        bridge.push_event(event);
    }

    // Return empty string for now, proper response handling would be async
    String::new()
}

// Bevy resource for managing the bridge
#[derive(Resource, Default)]
pub struct JsRustBridge {
    pub last_orientation: Option<DeviceOrientationData>,
    pub permission_status: PermissionStatus,
    pub events_processed: u32,
}

impl JsRustBridge {
    pub fn process_events(&mut self) -> Vec<BridgeEvent> {
        let mut events = Vec::new();

        if let Ok(mut bridge) = BRIDGE.lock() {
            while let Some(event) = bridge.pop_event() {
                match &event {
                    BridgeEvent::DeviceOrientation(data) => {
                        self.last_orientation = Some(data.clone());
                    }
                    BridgeEvent::PermissionStatusChanged(status) => {
                        self.permission_status = status.clone();
                    }
                    _ => {}
                }
                events.push(event);
                self.events_processed += 1;
            }
        }

        events
    }

    pub fn send_game_state_response(&self, state: &str, data: &str) {
        let event = BridgeEvent::GameStateResponse {
            state: state.to_string(),
            data: data.to_string(),
        };

        if let Ok(mut bridge) = BRIDGE.lock() {
            bridge.push_event(event);
        }
    }
}

// Bevy system for processing bridge events
pub fn process_bridge_events(
    mut bridge: ResMut<JsRustBridge>,
    mut tilt_input: ResMut<crate::input::TiltInput>,
) {
    let events = bridge.process_events();

    for event in events {
        match event {
            BridgeEvent::DeviceOrientation(data) => {
                // Update tilt input with device orientation data
                tilt_input.beta = data.beta;
                tilt_input.enabled = bridge.permission_status.granted;

                // Log throttled events (every 60 events ~1 second at 60Hz)
                if bridge.events_processed % 60 == 0 {
                    info!(
                        "Device orientation: beta={:.2}°, enabled={}",
                        data.beta, tilt_input.enabled
                    );
                }
            }
            BridgeEvent::PermissionStatusChanged(status) => {
                info!(
                    "Permission status changed: granted={}, available={}",
                    status.granted, status.available
                );
                tilt_input.enabled = status.granted;
            }
            BridgeEvent::GameStateRequest { state } => {
                info!("Game state requested: {}", state);
                // Handle game state requests here
            }
            BridgeEvent::GameStateResponse { state: _, data: _ } => {
                // This would typically be handled by JS side
            }
        }
    }
}
