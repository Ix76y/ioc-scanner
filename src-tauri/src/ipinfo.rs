use reqwest::{blocking::{Client, Response}, StatusCode, header::{HeaderMap, HeaderValue}};

#[path = "http.rs"] mod http; 


#[tauri::command]
pub fn get_ipinfo(ip: &str) -> Result<String, String> {
    return get(&ip);
}

fn get(ip: &str) -> Result<String, String> {
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_str("application/json").unwrap());
    let response = http::make_get_request(&format!("https://ipinfo.io/{}", ip), headers);
    let status = response.status();

    match status {
        StatusCode::OK => Ok(response.text().unwrap()),
        _ => Err(format!("Error in Response: {}", status)),
    }
}