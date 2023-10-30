use reqwest::Client;

pub fn create_request_instance() -> Client {
    Client::new()
}

pub fn generate_request_url(base_url: &str, path: &str) -> String {
    let mut base_url = base_url.to_string();
    let mut path = path.to_string();
    if base_url.ends_with('/') {
        base_url = base_url.strip_suffix("/").unwrap_or(&base_url).to_string()
    }
    if path.starts_with('/') {
        path = path.strip_prefix("/").unwrap_or(&path).to_string()
    }
    format!("{}/{}", base_url, path)
}