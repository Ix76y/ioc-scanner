use serde::{Deserialize, Serialize};
use serde_json::Result;

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
    status: Status
}

#[tauri::command]
pub fn get_integrations() -> [Integration; 7] {
    let integrations = [
        Integration {
            name: "URLScan.io".to_owned(),
            image: "urlscan-logo.png".to_owned(),
            url: "https://urlscan.io/".to_owned(),
            status: Status::InProgress
        },
        Integration {
            name: "EmailRep".to_owned(),
            image: "emailrep-logo.png".to_owned(),
            url: "https://emailrep.io/".to_owned(),
            status: Status::InProgress
        },
        Integration {
            name: "IPInfo.io".to_owned(),
            image: "ipinfo-logo.png".to_owned(),
            url: "https://ipinfo.io/".to_owned(),
            status: Status::InProgress
        },
        Integration {
            name: "VirusTotal".to_owned(),
            image: "vt-logo.png".to_owned(),
            url: "https://www.virustotal.com/gui/home/upload".to_owned(),
            status: Status::Planned
        },
        Integration {
            name: "IPLocation".to_owned(),
            image: "iplocation-logo.png".to_owned(),
            url: "https://www.iplocation.net/".to_owned(),
            status: Status::Planned
        },
        Integration {
            name: "AbuseIPDB".to_owned(),
            image: "abuseipdb-logo.png".to_owned(),
            url: "https://www.abuseipdb.com/".to_owned(),
            status: Status::Planned
        },
        Integration {
            name: "HaveIBeenPwned".to_owned(),
            image: "haveibeenpwned-logo.png".to_owned(),
            url: "https://haveibeenpwned.com/".to_owned(),
            status: Status::Planned
        },
    ];
    integrations
}
