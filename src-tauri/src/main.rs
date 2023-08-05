// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod urlscan;
mod ipinfo;

use crate::urlscan::*;
use crate::ipinfo::*;

/* 
#[tauri::command]
fn greet(name: &str) -> String {
   format!("Hello, {}!", name)
}
*/


fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_urlscan_quota, get_ipinfo])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
