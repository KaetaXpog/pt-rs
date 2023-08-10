use reqwest::{Client, IntoUrl, Url};
use std::time::Duration;
use std::{sync::Arc, thread::sleep};

use crate::ptsite::{read_cookies, Site};

const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36";

pub fn build_pt_client(site: Site) -> Client {
    let url: Url = site.url_site().parse().unwrap();
    let fpath = format!("./config/{}.cookies", site.to_string().to_lowercase());
    let jar = read_cookies(&fpath, &url);

    let jar = Arc::new(jar);
    let client = reqwest::Client::builder()
        .cookie_provider(jar)
        .user_agent(USER_AGENT)
        .brotli(true)
        .build()
        .unwrap();

    client
}

/// This client just has user_agent configured.
pub fn build_empty_client() -> Client {
    let client = reqwest::Client::builder()
        .user_agent(USER_AGENT)
        .brotli(true)
        .build()
        .unwrap();
    client
}

pub async fn get_html<U: IntoUrl>(client: &Client, url: U) -> Result<String, reqwest::Error> {
    client.get(url).send().await?.text().await
}

pub async fn get_html_with_retry<U: IntoUrl + Clone>(
    client: &Client,
    url: U,
    max_retry: u32,
) -> Result<String, reqwest::Error> {
    let mut res = get_html(client, url.clone()).await;
    for _ in 1..max_retry {
        match res {
            Ok(_) => return res,
            Err(_) => {
                sleep(Duration::from_millis(500));
                res = get_html(client, url.clone()).await
            }
        }
    }
    res
}
