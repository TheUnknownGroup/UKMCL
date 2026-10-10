use std::sync::LazyLock;
use reqwest::Client;

const USER_AGENT: &str = "UKMCL/1.0.0 (kaylordevon5@gmail.com)";

pub static DEFAULT: LazyLock<Client> = LazyLock::new(|| {
     Client::builder().user_agent(USER_AGENT).build().expect("Failed to create client")
});

pub fn base_client() -> Client {
     DEFAULT.clone()
}