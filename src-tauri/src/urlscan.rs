
use urlscan::UrlScanClient;

#[tauri::command]
pub fn get_urlscan_quota() -> String {
  let client = UrlScanClient::new("TODO");
  let response = client.get_quota();
  match response {
      Ok(quota) => format!("{:?}", quota.limits.public),
      _ => format!("{{\"day\": \"We got an error...\"}}"),
  }
}

#[tauri::command]
pub fn scan_url(url: &str, visibility: &str, tags: &str) -> String {
  // TODO: check if url is valid
  let tags: Vec<String> = tags.split(",").map(|v| v.to_string()).collect();
  let client = UrlScanClient::new("TODO");
  let response = client.scan_url(url, visibility, tags);
  match response {
      Ok(value) => format!("{:?}", value),
      _ => format!("Error scanning the URL"),
  }
}