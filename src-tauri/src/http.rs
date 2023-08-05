use reqwest::{blocking::{Client, Response}, header::HeaderMap};

static DEFAULT_USER_AGENT: &str = "curl/7.54.1"; //rust-client/urlscan+https://github.com/Ix76y/urlscan-rs";

pub(crate) fn make_get_request(url: &str, headers: HeaderMap) -> Response {
    let client = Client::builder().user_agent(DEFAULT_USER_AGENT).default_headers(headers).build().unwrap();
    return client.get(url).send().unwrap();
}