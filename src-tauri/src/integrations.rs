pub mod emailrep;
pub mod ipinfo;
pub mod urlscanio;


use serde::{Deserialize, Serialize};

use crate::secrets::{get_keys, get_secret};

const IMG_BY_NAME: &[(&str, &str)] = &[
    ("URLScan.io", "/src/lib/assets/urlscan-logo.png"),
    ("EmailRep", "/src/lib/assets/emailrep-logo.png"),
    ("IPInfo.io", "/src/lib/assets/ipinfo-logo.png"),
    ("VirusTotal", "/src/lib/assets/vt-logo.png"),
    ("IPLocation", "/src/lib/assets/iplocation-logo.png"),
    ("AbuseIPDB", "/src/lib/assets/abuseipdb-logo.png"), 
    ("HaveIBeenPwned", "/src/lib/assets/haveibeenpwned-logo.png"), 
    ("DomainTools", "/src/lib/assets/domaintools-logo.png")
];

pub fn get_img_by_name(key: &str) -> &str {
    IMG_BY_NAME.iter().find(|t| t.0 == key).unwrap().1
}


const DOMAIN_BY_NAME: &[(&str, &str)] = &[
    ("URLScan.io", "https://urlscan.io/"),
    ("EmailRep", "https://emailrep.io/"), 
    ("IPInfo.io", "https://ipinfo.io/"), 
    ("VirusTotal", "https://www.virustotal.com/gui/home/upload"),
    ("IPLocation", "https://www.iplocation.net/"),
    ("AbuseIPDB", "https://www.abuseipdb.com/"),
    ("HaveIBeenPwned", "https://haveibeenpwned.com/"),
    ("DomainTools", "https://domaintools.com/")
];

pub fn get_domain_by_name(key: &str) -> &str {
    DOMAIN_BY_NAME.iter().find(|t| t.0 == key).unwrap().1
}


#[derive(Serialize, Deserialize)]
pub enum Status {
    Planned,
    InProgress,
    Implemented,
    Requested
}


#[derive(Serialize, Deserialize, PartialEq)]
pub enum SecretType {
    Token,
    ApiKey,
    User
}

#[derive(Serialize, Deserialize)]
pub struct Integration {
    pub name: String,
    image: String, 
    url: String,
    status: Status,
    secret_type: SecretType,
    pub configured: bool,
    pub secret: String,
    pub username: Option<String>
}

fn build_integration(name: &str, status: Option<Status>, secret_type: Option<SecretType>) -> Integration {
    Integration {
        name: name.to_string(),
        image: get_img_by_name(name).to_string(),
        url: get_domain_by_name(name).to_string(),
        status: status.unwrap_or(Status::Planned),
        secret_type: secret_type.unwrap_or(SecretType::ApiKey),
        configured: false,
        secret: "".to_owned(),
        username: None
    }
}

#[tauri::command]
pub fn get_integrations() -> [Integration; 8] {
    let mut integrations = [
        build_integration("URLScan.io", Some(Status::InProgress), None),
        build_integration("EmailRep", Some(Status::Implemented), None),
        build_integration("IPInfo.io", Some(Status::InProgress), Some(SecretType::Token)),
        build_integration("VirusTotal", None, None),
        build_integration("IPLocation", None, None),
        build_integration("AbuseIPDB", None, None),
        build_integration("HaveIBeenPwned", None, None),
        build_integration("DomainTools", Some(Status::Planned), Some(SecretType::User))
    ];
    let configured_integrations =  get_keys(); // TODO: breaks in release, use: vec![]; to run release

    // update list of integrations with actual data
    for integration in integrations.iter_mut() {
        if configured_integrations.contains(&integration.name) {
            integration.configured = true;

            // get the secret of the integration and save it
            let secret = get_secret(&integration.name);
            if let Some(secret) = secret {
                integration.secret = secret;
            }

            // if the integration has username & password, also get the username
            if integration.secret_type == SecretType::User {
                let user: Option<String> = get_secret(&format!("{}-User", integration.name));
                integration.username = user;
            }
        }
    }
    integrations
}

