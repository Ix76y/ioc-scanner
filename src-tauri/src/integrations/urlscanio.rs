use std::thread;
use urlscan::UrlScanClient;

use crate::get_secret;


const URLSCAN_NAME: &str = "URLScan.io";

#[derive(Debug)]
struct UrlScan {
  api_key: String,
}

impl UrlScan {
  fn get_client(&self) -> UrlScanClient {
    return UrlScanClient::new(&self.api_key)
  }
}

#[tauri::command]
pub fn get_urlscan_quota() -> String {
  let api_key = String::from("TODO");
  let client = UrlScan{api_key}.get_client();
  let response = client.get_quota();
  match response {
      Ok(quota) => format!("{:?}", quota.limits.public),
      _ => format!("{{\"day\": \"We got an error...\"}}"),
  }
}

// #[tauri::command]
pub fn scan_url(url: &str, visibility: &str, tags: &str) -> Result<String, String> {
  let api_key = get_secret(URLSCAN_NAME);
  if let Some(api_key) = api_key {
    let tags: Vec<String> = tags.split(",").map(|v| v.to_string()).collect();
    let client = UrlScan{api_key}.get_client();
    let response = client.scan_url(url, visibility, tags);
    match response {
        Ok(value) => Ok(value.uuid),
        Err(e) => Err(format!("Error scanning the URL. Reason: {:?}", e)),
    }
  } else {
    return Err(format!("No URLScan.io API Key."));
  }
}

#[tauri::command]
pub fn get_urlscan_result(uuid: &str) -> String {
  let api_key = get_secret(URLSCAN_NAME);
  if let Some(api_key) = api_key {
    let client = UrlScan{api_key}.get_client();
    let response = client.get_result(uuid);
    match response {
        Ok(result) => return result,
        Err(e) => return format!("Something went wrong :( {}", e),
    }
  } else {
    return format!("No URLScan.io API Key.")
  }
}


#[tauri::command]
pub fn get_screenshot(uuid: &str) -> String {
  let api_key = String::from("TODO");
  let client = UrlScan{api_key}.get_client();
  let response = client.get_screenshot(uuid);
  match response {
      Ok(result) => {
          println!("All good?");
          _ = client.save_screenshot(result, "../urlscan.png");
          return String::from("../urlscan.png");
      },
      _ => println!("Didn't get the screenshot :("),
  };
  return String::from("Couldn't save screenshot.");
}