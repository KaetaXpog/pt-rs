use reqwest;
use tokio;

#[tokio::test]
async fn get_cat_fact() -> Result<(), reqwest::Error>{
    let client = reqwest::Client::new();
    let body = client.get("https://baidu.com").send()
        .await?
        .text()
        .await?;

    println!("{}", body);
    Ok(())
}

