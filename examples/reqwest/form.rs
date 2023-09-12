#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), reqwest::Error> {
    let response = reqwest::Client::new()
        .post("http://httpbin.org/post")
        .form(&[("one", "1")])
        .send()
        .await
        .expect("send");
    println!("Response status {}", response.status());
    let body = response.text().await?;
    println!("{}", body);
    Ok(())
}
