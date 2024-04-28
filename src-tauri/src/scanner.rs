use core::fmt;
use std::{collections::HashMap, iter::Scan};
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use validator::{validate_email, validate_ip, validate_url};

use crate::ipinfo::get_ipinfo;
use crate::urlscanio;

#[derive(Debug)]
enum Category {
    Ip,
    Domain,
    Url,
    Email
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Category::Ip => write!(f, "IP"),
            Category::Domain => write!(f, "Domain"),
            Category::Url => write!(f, "URL"),
            Category::Email => write!(f, "Email"),
        }
    }
}

impl FromStr for Category {
    type Err = (); 

    fn from_str(input: &str) ->Result<Category, Self::Err> {
        match input {
            "IP" => Ok(Category::Ip),
            "Domain" => Ok(Category::Domain),
            "URL" => Ok(Category::Url),
            "Email" => Ok(Category::Email),
            _ => Err(()),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ScanResult {
    successfull: bool,      // was the API call successull?
    integration: String,    // name of the integration returning the data
    result: String          // JSON from the API call
}

#[tauri::command]
pub fn scan(input: String, category: String) -> Result<Vec<ScanResult>, String> {
    let valid_input = validate_input(&input, &category);
    if !valid_input {
        return Err(format!("Invalid Input: Input doesn't match selected category {}.", category));
    }

    let mut results: Vec<ScanResult> = Vec::new();
    // let integrations = get_integrations_by_category().get(&category).unwrap();
    for integration in get_integrations_by_category().get(&category).unwrap() {
        match integration.as_str() {
            "IPInfo.io" => {
                let result = get_ipinfo(&input);
                match result {
                    Ok(result) => results.push(ScanResult {
                        successfull: true,
                        integration: integration.to_string(),
                        result: result
                    }),
                    Err(result) => results.push(ScanResult {
                        successfull: false,
                        integration: integration.to_string(),
                        result: result,
                    })
                };
            },
            "URLScan.io" => {
                let visibility: &str = "private";
                let tags: &str = "IOC-Scanner";
                let result = urlscanio::scan_url(&input, visibility, tags);
                match result {
                    Ok(result) => results.push(ScanResult { 
                        successfull: true, 
                        integration: integration.to_string(), 
                        result: result 
                    }),
                    Err(result) => results.push(ScanResult {
                        successfull: false,
                        integration: integration.to_string(),
                        result: result,
                    })
                };
            },
            _ => (),
        }
    }
    Ok(results)
}

fn validate_input(input: &str, category: &str) -> bool {
    let integrations_by_category = get_integrations_by_category();
    
    // check if category is valid
    if !integrations_by_category.contains_key(category) {
        return false;
    }

    // check if input is in the correct format for selected category
    // can directly unwrap here, cause we already checked if the category is valid
    let category: Category = Category::from_str(category).unwrap();
    match category {
        Category::Ip => validate_ip(input),
        Category::Domain => validate_url(input),
        Category::Email => validate_email(input),
        Category::Url => validate_url(input),
    }
}

fn get_integrations_by_category() -> HashMap<String, Vec<String>> {
    let mut integrations_by_cagtegory = HashMap::new();
    integrations_by_cagtegory.insert(Category::Ip.to_string(), vec![String::from("IPInfo.io"), String::from("VirusTotal"), String::from("IPLocation"), String::from("AbuseIPDB")]);
    integrations_by_cagtegory.insert(Category::Domain.to_string(), vec![String::from("URLScan.io"), String::from("VirusTotal"), String::from("Whois")]);
    integrations_by_cagtegory.insert(Category::Url.to_string(), vec![String::from("URLScan.io"), String::from("VirusTotal")]);
    integrations_by_cagtegory.insert(Category::Email.to_string(), vec![String::from("EmailRep"), String::from("HaveIBeenPwned")]);
    return integrations_by_cagtegory;
}