use reqwest::{blocking::Response, StatusCode, header::HeaderMap};
use urlencoding::encode;

#[path = "http.rs"] mod http; 

#[tauri::command]
pub fn get_emailrep(email: &str) -> String {
    let _encoded_email = encode(email);

    // TODO: check if API key exists and pass it in headers: https://docs.sublimesecurity.com/reference/emailrep-introduction
    let _headers = HeaderMap::new();
    return String::from("{\"email\": \"info@barracudabyte.de\", \"reputation\": \"none\", \"suspicious\": true, \"references\": 0, \"details\": {\"blacklisted\": false, \"malicious_activity\": false, \"malicious_activity_recent\": false, \"credentials_leaked\": false, \"credentials_leaked_recent\": false, \"data_breach\": false, \"first_seen\": \"never\", \"last_seen\": \"never\", \"domain_exists\": true, \"domain_reputation\": \"low\", \"new_domain\": false, \"days_since_domain_creation\": null, \"suspicious_tld\": false, \"spam\": false, \"free_provider\": false, \"disposable\": false, \"deliverable\": false, \"accept_all\": false, \"valid_mx\": true, \"primary_mx\": \"barracudabyte.de\", \"spoofable\": true, \"spf_strict\": false, \"dmarc_enforced\": false, \"profiles\": []}, \"logos\": [], \"company_logo\": \"\", \"explainer\": \"Suspicious. This email address is not deliverable, and the domain has low reputation. We have not observed this email address on the internet, and it has no profiles on major services like LinkedIn, Facebook, and iCloud. A lack of digital presence may simply indicate a new email address, but is typically suspicious. \"}");
    
    // emailrep has a very limited free api, commented out for testing...
    /*
    let response: Response = http::make_get_request(&format!("https://emailrep.io/query/{}", encoded_email), headers);
    let status: StatusCode = response.status();

    match status {
        StatusCode::OK => response.text().unwrap(),
        _ => format!("An Error occurred. Status {}. Message: {}", status.as_u16(), response.text().unwrap()),
    }
    */
}