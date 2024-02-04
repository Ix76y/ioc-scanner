// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod urlscan;
mod ipinfo;
mod emailrep;
mod secrets;
mod integrations;

use crate::urlscan::*;
use crate::ipinfo::*;
use crate::emailrep::*;
use crate::secrets::*;
use crate::integrations::*;


fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      get_urlscan_quota, 
      scan_url, 
      get_ipinfo, 
      get_emailrep, 
      has_secrets_store, 
      get_keys, 
      get_secret, 
      remove_secret,
      update_secret, 
      create_secrets_store,
      get_integrations
      ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
