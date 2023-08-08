use chrono::{DateTime, Local, TimeZone, Utc};
use itertools::Itertools;
use log::{debug, error};
use ptsite::Site;
use scraper::{ElementRef, Html, Selector};

pub mod client;
pub mod dataitem;
pub mod db;
pub mod ptsite;
pub mod scheduler;
pub mod utils;

// re-export some item
pub use dataitem::DataItem;

/// Assuming data present in `table.torrents` selector
pub fn process_table(html: &str, site: &Site) -> Vec<DataItem> {
    let document = Html::parse_document(html);
    let table_tag = Selector::parse("table.torrents").unwrap();
    // first table
    let table_html = document.select(&table_tag).next().unwrap();
    let table_html = table_html
        .select(&"tbody".try_into().unwrap())
        .next()
        .unwrap();
    // first table row
    let tr_tag = Selector::parse("tr").unwrap();
    let mut trs = table_html.select(&tr_tag);
    let tr_head = trs.next().unwrap();
    let headers = process_headers(tr_head);

    table_html
        .children()
        .skip(1)
        .map(|tr| {
            if !tr.value().is_element() {
                return None;
            }
            let data = process_row_data(site, ElementRef::wrap(tr).unwrap(), &headers);
            Some(data)
        })
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect_vec()
}

// This may be OPKT only
pub fn process_headers(tr_head: ElementRef) -> Vec<String> {
    let mut res = vec![];
    let td_tag = Selector::parse("td").unwrap();
    let img_tag = Selector::parse("img").unwrap();
    let a_tag = Selector::parse("a").unwrap();
    for td in tr_head.select(&td_tag) {
        let img = td.select(&img_tag).next();
        let a = td.select(&a_tag).next();
        let headername = match (img, a) {
            (None, None) => td.inner_html(),
            (Some(tag), _) => tag.value().attr("title").unwrap().to_owned(),
            (None, Some(a)) => a.inner_html(),
        };
        res.push(headername);
    }
    debug!("{}", tr_head.html());
    println!("{:?}", res);

    res
}

pub fn process_row_data(site: &Site, tr: ElementRef, headers: &Vec<String>) -> DataItem {
    let td_rowfollow = Selector::parse("td.rowfollow").unwrap();
    let url_site = site.url_site();

    let mut catogray: String = "".to_owned();
    let mut title: String = "".to_owned();
    let mut desc: String = "".to_owned();
    let mut finished:   u32 = 0;
    let mut download:   u32 = 0;
    let mut upload:     u32 = 0;
    let mut size:       f32 = 0.0;
    let mut publish_time: i64 = 0;
    let mut src_link: String = "".into();

    for (td, header) in tr.select(&td_rowfollow).zip(headers) {
        debug!("{} : {}", header, td.html());
        match header.as_ref() {
            "类型" => {
                catogray = td
                    .select(&Selector::parse("img").unwrap())
                    .next()
                    .unwrap()
                    .value()
                    .attr("title")
                    .unwrap()
                    .to_owned();
            }
            "标题" => {
                // The first td.embedded that contains an a tag, we call it TD,
                // title is the title attr value of the a tag in TD
                // desc is the last text node in the TD

                let selector = "td.embedded".try_into().unwrap();
                let td_embeddeds = td.select(
                    &selector);

                let first_td = td_embeddeds.into_iter().find(|&node| {
                        node.select(&"a".try_into().unwrap())
                        .next().is_some()
                }).unwrap();

                desc = first_td.text().last().unwrap().to_owned();

                let a_tag = first_td.select(
                    &"a".try_into().unwrap())
                    .next().unwrap();
                title = a_tag.value().attr("title").unwrap().to_owned();
                src_link = a_tag.value().attr("href").unwrap().into();

                src_link = url_site.to_string() + &src_link;
            }
            "完成数" => {
                let td_element = td.text().next()
                    // remove thousands seperator
                    .unwrap().replace(',', ""); 
                match td_element.parse() {
                    Ok(f) => finished = f,
                    Err(_) => {
                        error!("element {} cannot be parsed as u32 as finished",
                                td_element);
                        panic!();
                    }
                }
            }
            "种子数" => {
                upload = td.text().next().unwrap()
                    .replace(',', "").parse().unwrap();
            }
            "下载数" => {
                download = td.text().next().unwrap()
                    .replace(',', "").parse().unwrap();
            }
            "大小" => {
                let txt = td.text().collect_vec();
                size = txt[0].parse().unwrap();
                let unit = txt[1];
                match unit {
                    "MB" => size /= 1000.0,
                    "GB" => (),
                    "KB" => size /= 1000_000.0,
                    "TB" => size *= 1000.0,
                    x => panic!("Unexpected size unit {}", x),
                }
            }
            "存活时间" => {
                let time = td
                    .select(&"span".try_into().unwrap())
                    .next()
                    .unwrap()
                    .value()
                    .attr("title")
                    .unwrap();
                let time_with_zone = format!("{} {}", time, "+08:00");
                let datetime =
                    DateTime::parse_from_str(&time_with_zone, "%Y-%m-%d %H:%M:%S %z").unwrap();
                publish_time = datetime.timestamp();
            }
            _ => {}
        }
    }

    let time_str: DateTime<Local> = Utc.timestamp_opt(publish_time, 0).unwrap().into();
    println!(
        "{}, {}, \n\t{}, \n\t{}/{}/{}, {} GB, {} src: {}",
        catogray, title, desc, upload, download, finished, size, time_str, src_link
    );

    DataItem::new(
        &title,
        &desc,
        finished,
        download,
        upload,
        size,
        publish_time,
        &src_link,
    )
}
