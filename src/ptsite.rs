/// some pt site related logic
use crate::scheduler::Scheduler;
use crate::db;
use reqwest::{cookie::Jar, Url};
use std::io::Read;
use std::fs::File;
use std::str::FromStr;

/// This is the default time zone for all pt sites in China.
pub const TIME_ZONE: &str = "+08:00";

pub fn okpt_url_by_page(page: u32) -> String {
    format!(
        "https://www.okpt.net/torrents.php?inclbookmarked=0&incldead=1&spstate=0&page={}",
        page
    )
}

pub fn icc2022_url_by_page(page: u32) -> String {
    format!(
        "https://www.icc2022.com/torrents.php?inclbookmarked=0&incldead=1&spstate=0&cat409=1&cat405=1&cat404=1&cat401=1&page={}",
        page
    )
}

fn ggpt_url_by_page(page: u32) -> String{
    format!(
        "https://www.gamegamept.com/torrents.php?inclbookmarked=0&incldead=1&spstate=0&page={}",
        page
    )
}

fn carpt_url_by_page(page: u32) -> String{
    format!(
        "https://carpt.net/torrents.php?inclbookmarked=0&incldead=1&spstate=0&cat401=1&cat402=1&cat403=1&cat404=1&cat405=1&cat407=1&page={}",
        page
    )
}

fn pttime_url_by_page(page: u32) -> String{
    format!(
        "https://www.pttime.org/torrents.php?inclbookmarked=0&incldead=1&spstate=0&page={}",
        page
    )
}

pub fn read_cookies(fpath: &str, url: &Url) -> Jar{
    let mut s: String = String::new();
    let mut reader = File::open(fpath).unwrap();
    reader.read_to_string(&mut s).unwrap();

    let jar = Jar::default();
    for subs in s.split(";"){
        jar.add_cookie_str(subs, url)
    }
    jar
}


#[derive(Debug, Copy, Clone)]
pub enum Site {
    OKPT,
    ICC2022,
    PTTIME,
    GGPT,
    CARPT,
}

impl Site{
    pub fn url_site(&self) -> String{
        let res = match self {
            Site::OKPT => "https://www.okpt.net/",
            Site::ICC2022 => "https://www.icc2022.com/",
            Site::GGPT => "https://www.gamegamept.com/",
            Site::CARPT => "https://carpt.net/",
            Site::PTTIME => "https://www.pttime.org/",
        };
        res.to_owned()
    }
}

impl ToString for Site{
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

impl FromStr for Site{
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_ref() {
            "icc2022" | "icc" => Ok(Site::ICC2022), 
            "okpt" => Ok(Site::OKPT),
            "ggpt" => Ok(Site::GGPT),
            "carpt" => Ok(Site::CARPT),
            "pttime" => Ok(Site::PTTIME),
            _ => Err("ParseSiteError".into())
        }
    }
}

#[derive(Debug)]
pub enum Discount {
    DoubleFree,
    Free,
    R25,
    FiftyPerOff,
}

pub async fn scrape_icc2022(start: u32, end: u32, db_name: &str) {
    let mut sche = Scheduler::new(Site::ICC2022, start, end, icc2022_url_by_page);

    let conn = db::create_table(db_name);
    sche.finish_the_work(&conn).await;
}

/// start page number and end page number, both included. Start from 0
pub async fn scrape_okpt(start: u32, end: u32, db_name: &str){
    let mut sche = Scheduler::new(
        Site::OKPT, start, end, okpt_url_by_page
    );

    let conn = db::create_table(db_name);
    sche.finish_the_work(&conn).await;
}

pub async fn scrape_ggpt(start: u32, end: u32, db_name: &str){
    let mut sche = Scheduler::new(
        Site::GGPT, start, end, ggpt_url_by_page
    );
    let conn = db::create_table(db_name);
    sche.finish_the_work(&conn).await;
}

pub async fn scrape_carpt(start: u32, end: u32, db_name: &str){
    let mut sche = Scheduler::new(
        Site::CARPT, start, end, carpt_url_by_page
    );
    let conn = db::create_table(db_name);
    sche.finish_the_work(&conn).await;
}

pub async fn scrape_pttime(start: u32, end: u32, db_name: &str){
    let mut sche = Scheduler::new(
        Site::PTTIME, start, end, pttime_url_by_page
    );
    let conn = db::create_table(db_name);
    sche.finish_the_work(&conn).await;
}

#[test]
fn test_read_cookies(){
    let url = "https://carpt.net/".parse().unwrap();
    let jar = read_cookies("./config/carpt.cookies", &url);
    println!("{:?}", jar);
}
