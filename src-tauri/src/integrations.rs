use serde::{Deserialize, Serialize};
use serde_json::Result;

use crate::secrets::{get_keys, get_secret};

#[derive(Serialize, Deserialize)]
pub enum Status {
    Planned,
    InProgress,
    Implemented,
    Requested
}

#[derive(Serialize, Deserialize)]
pub struct Integration {
    pub name: String,
    image: String, 
    url: String,
    status: Status,
    pub configured: bool,
    pub apikey: String
}

fn build_integration(name: String, image: String, url: String, status: Option<Status>) -> Integration {
    Integration {
        name,
        image,
        url,
        status: status.unwrap_or(Status::Planned),
        configured: false,
        apikey: "".to_owned()
    }
}

#[tauri::command]
pub fn get_integrations() -> [Integration; 7] {
    let mut integrations = [
        build_integration("URLScan.io".to_owned(), "/src/lib/assets/urlscan-logo.png".to_owned(), "https://urlscan.io/".to_owned(), Some(Status::InProgress)),
        build_integration("EmailRep".to_owned(), "/src/lib/assets/emailrep-logo.png".to_owned(), "https://emailrep.io/".to_owned(), Some(Status::InProgress)),
        build_integration("IPInfo.io".to_owned(), "/src/lib/assets/ipinfo-logo.png".to_owned(), "https://ipinfo.io/".to_owned(), Some(Status::InProgress)),
        build_integration("VirusTotal".to_owned(), "/src/lib/assets/vt-logo.png".to_owned(), "https://www.virustotal.com/gui/home/upload".to_owned(), None),
        build_integration("IPLocation".to_owned(), "/src/lib/assets/iplocation-logo.png".to_owned(), "https://www.iplocation.net/".to_owned(), None),
        build_integration("AbuseIPDB".to_owned(), "/src/lib/assets/abuseipdb-logo.png".to_owned(), "https://www.abuseipdb.com/".to_owned(), None),
        build_integration("HaveIBeenPwned".to_owned(), "/src/lib/assets/haveibeenpwned-logo.png".to_owned(), "https://haveibeenpwned.com/".to_owned(), None)
    ];
    let configured_integrations = get_keys();
    for integration in integrations.iter_mut() {
        if configured_integrations.contains(&integration.name) {
            integration.configured = true;
            let secret = get_secret(&integration.name);
            integration.apikey = secret;
        }
    }
    integrations
}
