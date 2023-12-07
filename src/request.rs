use reqwest;
use clap::ColorChoice;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // Specify the URL you want to make a request to
    let url = "https://httpbin.org/ip";

    // Make the HTTP GET request
    let response = reqwest::get(url).await?;

    // Check if the request was successful (HTTP status code 200 OK)
    if response.status().is_success() {
        // Print the response body as a string
        let body = response.text().await?;
        println!("Response body: {}", body);
    } else {
        println!("Request failed with status code: {}", response.status());
    }

    Ok(())
}
