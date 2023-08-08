use env_logger;
use pt_rs::ptsite::{self};

async fn one_page_for_all_site(db_name: &str){
    ptsite::scrape_okpt(0, 0, db_name).await;
    ptsite::scrape_icc2022(0, 0, db_name).await;
    ptsite::scrape_ggpt(0, 0, db_name).await;
    ptsite::scrape_carpt(0, 0, db_name).await;
    ptsite::scrape_pttime(0, 0, db_name).await;
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let db_name = "./data/pts.sqlite";

    // one_page_for_all_site(db_name).await;
    // ptsite::scrape_okpt(0, 62, db_name).await;
    // ptsite::scrape_icc2022(0, 57, db_name).await;
    // ptsite::scrape_ggpt(0, 14, db_name).await;
    //  ptsite::scrape_carpt(133, 133, db_name).await;
    ptsite::scrape_pttime(0, 753, db_name).await;
}
