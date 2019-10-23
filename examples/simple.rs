#![deny(warnings)]
#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let res = reqwest::Client::new()
        .get("https://hyper.rs")
        .send()
        .await?;

    println!("Status: {}", res.status());

    let body = res.text().await?;

    println!("Body:\n\n{}", body);

    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn main() {}
