#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), reqwest::Error> {
    let echo_json: serde_json::Value = reqwest::Client::new()
        .post("http://jsonplaceholder.typicode.com/posts")
        .json(&serde_json::json!({
            "title": "Reqwest.rs",
            "body": "http://docs.rs/reqwest",
            "userId": 1
        }))
        .send()
        .await?
        .json()
        .await?;

    println!("{:#?}", echo_json);
    // Object(
    //     {
    //         "body": String(
    //             "https://docs.rs/reqwest"
    //         ),
    //         "id": Number(
    //             101
    //         ),
    //         "title": String(
    //             "Reqwest.rs"
    //         ),
    //         "userId": Number(
    //             1
    //         )
    //     }
    // )
    Ok(())
}
