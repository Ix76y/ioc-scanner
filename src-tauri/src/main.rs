// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod secrets;
mod scanner;
mod integrations; 
mod http;

use tauri_plugin_store::{StoreBuilder, Builder};
use serde_json::json;

use crate::urlscanio::*;
use crate::ipinfo::*;
use crate::emailrep::*;
use crate::secrets::*;
use crate::integrations::*;
use crate::scanner::*;



fn main() {
  tauri::Builder::default()
    .plugin(Builder::default().build())
    .setup(|app| {
      // create store for user data
      let mut store = StoreBuilder::new(app.handle(), "ioc-scanner.bin".parse()?).build();
      //store.load();
      //store.delete("a");
      // store.insert("a".to_string(), json!("b"));
      store.save();
      return Ok(())

    })
    .invoke_handler(tauri::generate_handler![
      get_urlscan_quota, 
      get_urlscan_result,
      get_ipinfo, 
      get_emailrep, 
      has_secrets_store, 
      get_keys, 
      get_secret, 
      remove_secret,
      update_secret, 
      create_secrets_store,
      get_integrations,
      scan
      ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
