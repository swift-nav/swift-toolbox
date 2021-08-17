#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use console_backend::server::{Server, ServerEndpoint};
struct AppState {
  server: Server,
  endpoint: ServerEndpoint,
}

#[tauri::command]
fn write_log_message(message: String) {
  eprintln!("JS: {}", message);
}

#[tauri::command]
fn fetch_ipc_message(state: tauri::State<AppState>) -> Result<Option<(u8, Vec<u8>)>, String> {
  state.server.fetch_message()
    .map_err(|e| format!("{}", e))
}

fn main() {

  let mut server = Server::new();
  let endpoint = server.start();

  let state = AppState { server, endpoint };

  tauri::Builder::default()
    .manage(state)
    .invoke_handler(tauri::generate_handler![write_log_message, fetch_ipc_message])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
