use env_logger;
use pt_rs::{ptsite::{self}, client};
use clap::{Parser, Subcommand};
use pt_rs::ptsite::Site;
use pt_rs::utils::SaveTo;


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    site: Site,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Sample{
        name: Option<String>,   // save to ./data/{name}.html
        #[arg(long)]
        url: Option<String>
    } ,
    Scrape{
        start: u32,
        end: u32
    },
    Free,
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let db_name = "./data/pts.sqlite";

    let cli = Cli::parse();

    match cli.command {
        Commands::Sample { name, url }=> {
            let client = client::build_pt_client(cli.site);
            let url = match url {
                None => format!("{}/torrents.php", cli.site.url_site()),
                Some(s) => s
            };
            
            let html = client::get_html(&client, &url).await.unwrap();
            let name = match name {
                None => format!("./data/{}.html", cli.site.to_string()),
                Some(s) => format!("./data/{}.html", s)
            };
            html.save_to(&name);
            println!("SAVE {} to {}", url, name);
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
        Commands::Free => {
            let site = cli.site;
            for start in site.free_starts(){
                ptsite::scrape_pt_site(site, start, db_name).await;
            }
        }
    }
}
