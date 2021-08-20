#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::sync::Mutex;

use console_backend::{server::{Server, ServerEndpoint}, types};
struct AppState {
  server: Server,
  endpoint: ServerEndpoint,
}

#[tauri::command]
fn write_log_message(message: String) {
  eprintln!("JS: {}", message);
}

#[tauri::command]
fn send_ipc_message(state: tauri::State<Mutex<AppState>>, buffer: Vec<u8>) -> Result<(), String> {
  let mut state = state.lock().unwrap();
  state.endpoint.send_message(types::IPC_KIND_CBOR, buffer.to_vec())
    .map_err(|e| format!("{}", e))
}

#[tauri::command]
fn fetch_ipc_message(state: tauri::State<Mutex<AppState>>) -> Result<Option<(u8, Vec<u8>)>, String> {
  let state = state.lock().unwrap();
  state.server.fetch_message()
    .map_err(|e| format!("{}", e))
}

fn main() {

  let mut server = Server::new();
  let endpoint = server.start();

  let state = Mutex::new(AppState { server, endpoint });

  tauri::Builder::default()
    .manage(state)
    .invoke_handler(tauri::generate_handler![write_log_message, fetch_ipc_message, send_ipc_message])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
