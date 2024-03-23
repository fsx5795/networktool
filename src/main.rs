#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod cmd;
fn main() -> () {
    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![cmd::get_ipaddr])
    .run(tauri::generate_context!()).unwrap();
}