use serde::Deserialize;
use reqwest::header::USER_AGENT;
use env_logger;
use log::error;

#[derive(Deserialize, Debug)]
struct User {
    login: String,
    id: u32,
    node_id: String,
    url: String,
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let request_url = format!("https://api.github.com/repos/{owner}/{repo}/stargazers", owner = "rust-lang-nursery", repo = "rust-cookbook");
    println!("{}", request_url);
    let client = reqwest::Client::new();

    match client
        .get(&request_url)
        .header(USER_AGENT, "rust web-api-client demo")
        .send()
        .await
    {
        Ok(response) => {
            match response.json::<Vec<User>>().await {
                Ok(users) => println!("{:#?}", users),
                Err(err) => {
                    error!("Error while parsing json: {}",err);
                }
            }
        }
        Err(err) => {
            error!("Error while fetching data from url: {}",err);
        }
    }
}
