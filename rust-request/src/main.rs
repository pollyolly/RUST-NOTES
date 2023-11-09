use serde::Deserialize;
use reqwest::Error;
use reqwest::header::USER_AGENT;

#[derive(Deserialize, Debug)]
struct User{
    login:String,
    id: u32,
}

#[tokio::main]
async fn main() -> Result<(), Error>{
    let ownerstr = "pollyolly";
    let repostr = "OPENCV-NOTES";
    let request_url = format!("https://api.github.com/repos/{owner}/{repo}/stargazers",
                              owner = ownerstr,
                              repo = repostr);
    println!("{}", request_url);
    let client = reqwest::Client::new();
    let response = client.get(&request_url).header(USER_AGENT,"this is a sample message").send().await?;
    let users: Vec<User> = response.json().await?;
    println!("{:?}", users);
    Ok(())
}