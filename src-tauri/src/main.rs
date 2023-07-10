// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use urlscan::UrlScanClient;


#[tauri::command]
fn greet(name: &str) -> String {
   format!("Hello, {}!", name)
}


#[tauri::command]
fn get_urlscan_quota() -> String {
  let client = UrlScanClient::new("TODO");
  let response = client.get_quota();
  match response {
      Ok(quota) => format!("{:?}", quota.limits.public),
      _ => format!("We got an error..."),
  }
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_urlscan_quota])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
