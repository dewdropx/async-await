use error_chain::error_chain; // error handling

// handle errors
error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main] // setup tokio runtime
// define async main function
async fn main() -> Result<()> {

    // async get request and read response
    let response = reqwest::get("http://httpbin.org/get").await?;
    println!("Status: {}", response.status());
    println!("Headers:\n{:#?}", response.headers());

    let body = response.text().await?;
    println!("Body:\n{}", body);

    Ok(())
}