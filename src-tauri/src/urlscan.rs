
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