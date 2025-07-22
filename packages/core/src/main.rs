// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod components;
mod config;
mod engine;
mod systems;
mod utils;

use components::dto::{StatsDto, WorldDto};
use engine::simulation::Simulation;
use std::sync::{Arc, Mutex};
use tauri::Emitter;

pub struct AppState(Arc<Mutex<Option<Simulation>>>);

fn main() {
    let app_state = AppState(Arc::new(Mutex::new(None)));

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            initialise_simulation,
            get_world_state,
            get_world_statistics
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn initialise_simulation(
    device_width: u32,
    device_height: u32,
    app_handle: tauri::AppHandle,
    app_state: tauri::State<AppState>,
) {
    let sim_width = device_width as f32 * 1.5;
    let sim_height = device_height as f32 * 1.5;

    println!("Initialising world with size: {}x{}", sim_width, sim_height);

    let mut world = Simulation::new(sim_width, sim_height);
    let initial_world_state = world
        .get_world_state_dto()
        .expect("Failed to get initial world state");

    if let Err(e) = app_handle.emit("sim-initialised", initial_world_state) {
        eprintln!("Error emitting sim-initialised event: {}", e);
    }

    *app_state.0.lock().unwrap() = Some(world);
}

#[tauri::command]
fn get_world_state(app_state: tauri::State<AppState>) -> Result<Option<WorldDto>, String> {
    let mut state_guard = app_state.0.lock().unwrap();

    // If the world is initialised, tick it and return the state
    if let Some(world) = state_guard.as_mut() {
        world.tick();
        let world_dto = world
            .get_world_state_dto()
            .expect("Failed to get world state DTO");
        Ok(Some(world_dto))
    } else {
        Ok(None)
    }
}

#[tauri::command]
fn get_world_statistics(app_state: tauri::State<AppState>) -> Result<Option<StatsDto>, String> {
    let mut state_guard = app_state.0.lock().unwrap();

    // If the world is initialised, tick it and return the state
    if let Some(world) = state_guard.as_mut() {
        let stats_dto = world
            .get_world_statistics_dto()
            .expect("Failed to get statistics DTO");
        Ok(Some(stats_dto))
    } else {
        Ok(None)
    }
}
