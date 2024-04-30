use reqwest::{blocking::{Client, Response}, StatusCode, header::{HeaderMap, HeaderValue}};

use crate::{get_secret, http, ScanResult};


//#[path = "http.rs"] mod http; 

const IPINFO_NAME: &str = "IPInfo.io";

#[tauri::command]
pub fn get_ipinfo(ip: &str) -> ScanResult {
    let result = get(&ip);
    match result {
        Ok(result) =>  return ScanResult {
            successfull: true,
            integration: IPINFO_NAME.to_string(),
            result: result
        },
        Err(result) => return ScanResult {
            successfull: false,
            integration: IPINFO_NAME.to_string(),
            result: result,
        }
    };
}

fn get(ip: &str) -> Result<String, String> {
    let token = get_secret(IPINFO_NAME);
    if let Some(token) = token {
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_str("application/json").unwrap());
        let response = http::make_get_request(&format!("https://ipinfo.io/{}?token={}", ip, token), headers);
        let status = response.status();
    
        match status {
            StatusCode::OK => Ok(response.text().unwrap()),
            _ => Err(format!("Error in Response: {}", status)),
        }
    } else {
        return Err(format!("No IPInfo token."));
    }
}