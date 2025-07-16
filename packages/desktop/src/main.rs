// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Import your simulation code
mod components;
mod dto;
mod maths;
mod simulation;
mod systems;

use simulation::Simulation;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tauri::{Emitter};
use dto::WorldDto;

pub struct AppState {
    simulation: Arc<Mutex<Simulation>>,
}

fn main() {
    let app_state = AppState {
        simulation: Arc::new(Mutex::new(Simulation::new())),
    };
    let sim_for_thread = app_state.simulation.clone();

    tauri::Builder::default()
        .manage(app_state)
        .setup(|app| {
            let app_handle = app.handle().clone();
            thread::spawn(move || loop {
                sim_for_thread.lock().unwrap().tick();
                app_handle.emit("world_update", ()).unwrap();
                thread::sleep(Duration::from_millis(50));
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_world_state])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_world_state(state: tauri::State<AppState>) -> Result<WorldDto, String> {
    state.simulation.lock().unwrap().get_world_state_dto().map_err(|e| e.to_string())
}
