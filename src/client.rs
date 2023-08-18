use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{header, Client, IntoUrl, Url};
use std::time::Duration;
use std::{sync::Arc, thread::sleep};

use crate::ptsite::{read_cookies, Site};

const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36";

fn extra_headers() -> HeaderMap {
    let mut headers = header::HeaderMap::new();
    headers.insert("Accept", HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7"));
    headers.insert(
        "Sec-Ch-Ua",
        HeaderValue::from_static(
            r#""Not/A)Brand";v="99", "Google Chrome";v="115", "Chromium";v="115""#,
        ),
    );
    headers.insert("Sec-Ch-Ua-Mobile", HeaderValue::from_static("?0"));
    headers.insert(
        "Sec-Ch-Ua-Platform",
        HeaderValue::from_static(r#""Windows""#),
    );
    headers.insert("Sec-Fetch-Dest", HeaderValue::from_static("document"));
    headers.insert("Sec-Fetch-Mode", HeaderValue::from_static("navigate"));
    headers.insert("Sec-Fetch-Site", HeaderValue::from_static("none"));
    headers.insert("Sec-Fetch-User", HeaderValue::from_static("?1"));
    headers.insert("Upgrade-Insecure-Requests", HeaderValue::from_static("1"));
    headers
}

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
