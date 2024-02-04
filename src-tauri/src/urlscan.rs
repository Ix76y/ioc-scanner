
use urlscan::UrlScanClient;

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

#[tauri::command]
pub fn scan_url(url: &str, visibility: &str, tags: &str) -> String {
  // TODO: check if url is valid
  let tags: Vec<String> = tags.split(",").map(|v| v.to_string()).collect();
  let api_key = String::from("TODO");
  let client = UrlScan{api_key}.get_client();
  let response = client.scan_url(url, visibility, tags);
  match response {
      Ok(value) => format!("{:?}", value),
      _ => format!("Error scanning the URL"),
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