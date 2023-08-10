use chrono::{Utc, Months};
use crate::ptsite::Discount;

#[derive(Debug)]
pub struct DataItem{
    pub title:          String,
    pub desc :          String,
    pub finished:       u32,
    pub download:       u32,
    pub upload:         u32,
    // in Gigabytes
    pub size:           f32,
    pub publish_time:   i64,
    pub last_update:    i64,
    pub src_link:       String,

    // discount
    pub discount:       Option<Discount>,
    pub discount_due:   Option<i64>
}

impl DataItem{
    pub fn new(title: &str, desc: &str, finished: u32, download: u32,
    upload: u32, size: f32, publish_time: i64, src_link: &str, 
    discount: Option<Discount>, discount_due: Option<i64>
) -> Self{
        let now = Utc::now();
        let due: Option<i64> = match discount{
            Some(_) => match discount_due{
                x @ Some(_) => x,
                None => now.checked_add_months(Months::new(3)).unwrap().timestamp().into()
            }
            None => None
        };
        DataItem { title: title.to_owned(), desc: desc.to_owned(), finished,    
            download, upload, size, publish_time, last_update: now.timestamp(),
            src_link: src_link.to_owned(), discount, discount_due: due }
    }
}

