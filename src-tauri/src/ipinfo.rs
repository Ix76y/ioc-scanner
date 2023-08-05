use reqwest::{blocking::{Client, Response}, StatusCode, header::{HeaderMap, HeaderValue}};

#[path = "http.rs"] mod http; 


#[tauri::command]
pub fn get_ipinfo() -> String {
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_str("application/json").unwrap());
    let response = http::make_get_request("https://ipinfo.io/", headers);
    let status = response.status();

    match status {
        StatusCode::OK => response.text().unwrap(),
        _ => format!("None 200 status."),
    }
}