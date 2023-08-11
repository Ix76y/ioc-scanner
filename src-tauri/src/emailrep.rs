use reqwest::{blocking::Response, StatusCode, header::HeaderMap};
use urlencoding::encode;

#[path = "http.rs"] mod http; 

#[tauri::command]
pub fn get_emailrep(email: &str) -> String {
    let encoded_email = encode(email);

    // TODO: check if API key exists and pass it in headers: https://docs.sublimesecurity.com/reference/emailrep-introduction
    let headers = HeaderMap::new();
    let response: Response = http::make_get_request(&format!("https://emailrep.io/query/{}", encoded_email), headers);
    let status: StatusCode = response.status();

    match status {
        StatusCode::OK => response.text().unwrap(),
        _ => format!("An Error occurred. Status {}. Message: {}", status.as_u16(), response.text().unwrap()),
    }
}