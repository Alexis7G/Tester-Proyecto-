// Oculta la consola negra en Windows al abrir la app
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Serialize;
use sysinfo::System;

#[derive(Serialize)]
struct SystemInfo {
    total_ram_mb: u64,
    free_ram_mb: u64,
    cpu_cores: usize,
}

// El frontend la llama con: invoke('system_info')
// Sirve para recomendar la RAM del juego según tu PC.
#[tauri::command]
fn system_info() -> SystemInfo {
    let mut sys = System::new();
    sys.refresh_memory();
    SystemInfo {
        total_ram_mb: sys.total_memory() / 1024 / 1024,
        free_ram_mb: sys.available_memory() / 1024 / 1024,
        cpu_cores: std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![system_info])
        .run(tauri::generate_context!())
        .expect("error al iniciar TAVOX LCH");
}
