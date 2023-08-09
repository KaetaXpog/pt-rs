use env_logger;
use pt_rs::{ptsite::{self}, client};
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use pt_rs::ptsite::Site;
use pt_rs::utils::SaveTo;

async fn one_page_for_all_site(db_name: &str){
    ptsite::scrape_okpt(0, 0, db_name).await;
    ptsite::scrape_icc2022(0, 0, db_name).await;
    ptsite::scrape_ggpt(0, 0, db_name).await;
    ptsite::scrape_carpt(0, 0, db_name).await;
    ptsite::scrape_pttime(0, 0, db_name).await;
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    site: Site,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Sample ,
    Scrape{
        start: u32,
        end: u32
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let db_name = "./data/pts.sqlite";

    let cli = Cli::parse();

    match cli.command {
        Commands::Sample => {
            let client = client::build_pt_client(cli.site);
            let url = format!("{}/torrents.html", cli.site.url_site());
            client::get_html(&client, &url).await.unwrap()
            .save_to(&format!("./data/{}_torrents.html", 
                cli.site.to_string()));
        }
        Commands::Scrape { start, end } => {
            match cli.site {
                Site::OKPT => ptsite::scrape_okpt(start, end, db_name).await,
                Site::GGPT => ptsite::scrape_ggpt(start, end, db_name).await,
                Site::CARPT => ptsite::scrape_carpt(start, end, db_name).await,
                Site::PTTIME => ptsite::scrape_pttime(start, end, db_name).await,
                Site::ICC2022 => ptsite::scrape_icc2022(start, end, db_name).await
            }
        }
    }

    // one_page_for_all_site(db_name).await;
}
