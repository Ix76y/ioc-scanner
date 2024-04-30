use reqwest::{blocking::Response, header::{HeaderMap, HeaderValue}, StatusCode};
use urlencoding::encode;

use crate::{get_secret, http, ScanResult};

// #[path = "http.rs"] mod http; 

const EMAILREP_NAME: &str = "EmailRep";

#[tauri::command]
pub fn get_emailrep(email: &str) -> ScanResult {
    let result = get(&email);
    match result {
        Ok(result) =>  return ScanResult {
            successfull: true,
            integration: EMAILREP_NAME.to_string(),
            result: result
        },
        Err(result) => return ScanResult {
            successfull: false,
            integration: EMAILREP_NAME.to_string(),
            result: result,
        }
    };
}

fn get(email: &str) -> Result<String, String> {
    let apikey = get_secret(EMAILREP_NAME);
    if let Some(apikey) = apikey {
        let encoded_email = encode(email);

        // updating headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_str("application/json").unwrap());
        headers.insert("Key", HeaderValue::from_str(&apikey).unwrap());

        // request
        let response: Response = http::make_get_request(&format!("https://emailrep.io/query/{}", encoded_email), headers);

        // check response
        let status = response.status();
        match status {
            StatusCode::OK => Ok(response.text().unwrap()),
            _ => Err(format!("Error in Response: {}", status)),
        }
    } else {
        return Err(format!("No EmailRep API Key."));
    }
}

/* 
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
*/