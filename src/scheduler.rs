use crate::client;
use crate::db;
use crate::process_table;
use crate::ptsite::Site;
/// scrape okpt site by page
use crate::DataItem;
use reqwest::Client;
use sqlite::Connection;
use std::{thread::sleep, time::Duration};

pub struct Scheduler<F: Fn(u32) -> String> {
    site: Site,
    page: u32,     // next page to be scraped
    max_page: u32, // last page
    client: Client,
    gen_page: F,
}

/// this is site agnostic
impl<F: Fn(u32) -> String> Scheduler<F> {
    pub fn new(site: Site, page: u32, max_page: u32, gen_page: F) -> Self {
        let client = client::build_pt_client(site);
        Self {
            site, page, max_page,
            client, gen_page,
        }
    }

    pub async fn do_next_page(&mut self) -> Vec<DataItem> {
        let url = (self.gen_page)(self.page);
        let html = client::get_html_with_retry(&self.client, &url, 3)
            .await
            .unwrap();
        let items = process_table(&html, &self.site);

        // update state
        self.page += 1;

        items
    }

    pub async fn finish_the_work(&mut self, conn: &Connection) {
        while !self.is_finished() {
            let (i, max) = self.progress();
            println!("scraping {} of {} from {:?}", i, max, self.site);

            let items = self.do_next_page().await;
            println!(".... get {} items, updating", items.len());

            db::insert_or_update_batch(&conn, items);
            sleep(Duration::from_secs(2));
        }
        println!("finish {:?}\n\n", self.site)
    }

    pub fn is_finished(&self) -> bool {
        self.page > self.max_page
    }

    pub fn progress(&self) -> (u32, u32) {
        (self.page, self.max_page)
    }
}
