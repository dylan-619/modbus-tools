pub mod error;
pub mod models;
pub mod parser;
pub mod protocol;
pub mod connection;
pub mod commands;
pub mod state;

use crate::state::AppState;
use crate::commands::{connect_modbus, disconnect_modbus, read_value, write_value, list_serial_ports};
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            app.manage(AppState::new());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            connect_modbus,
            disconnect_modbus,
            read_value,
            write_value,
            list_serial_ports
        ])
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { .. } = event {
                let state = window.state::<AppState>();
                state.shutdown();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
