// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod components;
mod config;
mod engine;
mod systems;
mod utils;

use components::dto::{StatsDto, WorldDto};
use engine::simulation::{Simulation, SimulationOptions};
use std::sync::{Arc, Mutex};
use tauri::Emitter;
use tauri_plugin_log::{Builder as LogBuilder, Target, TargetKind};

pub struct AppState(Arc<Mutex<Option<Simulation>>>);

fn main() {
    let app_state = AppState(Arc::new(Mutex::new(None)));

    let log_plugin = LogBuilder::new()
        .targets([
            Target::new(TargetKind::Stdout),
            Target::new(TargetKind::LogDir {
                file_name: Some("app".into()),
            }),
            Target::new(TargetKind::Webview),
        ])
        .level(log::LevelFilter::Info)
        .build();

    tauri::Builder::default()
        .plugin(log_plugin)
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
    init_params: SimulationOptions,
    app_handle: tauri::AppHandle,
    app_state: tauri::State<AppState>,
) {
    log::info!("Received initialisation parameters: {:?}", init_params);

    let mut sim_params = init_params;
    sim_params.width *= 1.5;
    sim_params.height *= 1.5;

    log::info!("Initialising world with parameters: {:?}", sim_params);

    let mut world = Simulation::new(sim_params);
    let initial_world_state = world
        .get_world_state_dto()
        .expect("Failed to get initial world state");

    if let Err(e) = app_handle.emit("sim-initialised", initial_world_state) {
        log::error!("Error emitting sim-initialised event: {e}");
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
