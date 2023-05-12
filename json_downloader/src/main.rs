use std::{fs::File, io::{self, Write}};

static APP_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/94.0.4606.61 Safari/537.36";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Write the subreddit:");
    let mut subreddit = String::new();
    io::stdin()
        .read_line(&mut subreddit)
        .expect("Failed to read the line");

    // Make the HTTP request to fetch the data
    let client = get_client().build()?;
    let url = format!("https://www.reddit.com/r/{}/top.json", subreddit);
    let response = client.get(&url).send().await?;

    // Retrieve the response body as a string
    let body = response.text().await?;

    println!("Response body: {}", body);

    // Parse the JSON response
    let res: serde_json::Value = serde_json::from_str(&body)?;

    // Write the response to response.json
    let mut file = File::create("response.json")?;
    let json_str = serde_json::to_string(&res)?;
    file.write_all(json_str.as_bytes())?;

    if let Ok(_file) = File::open("response.json") {
        println!("Response successfully written to response.json");
    } else {
        println!("response.json does not found");
    }
    Ok(())
}

fn get_client() -> reqwest::ClientBuilder {
    reqwest::Client::builder().user_agent(APP_USER_AGENT)
}
