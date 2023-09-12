use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Post {
    id: Option<i32>,
    title: String,
    body: String,
    #[serde(rename = "userId")]
    user_id: i32,
}

// This is using the `tokio` runtime. You'll need the following dependency:
//
// `tokio = { version = "1", features = ["full"] }`
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), reqwest::Error> {
    let new_post = Post {
        id: None,
        title: "Reqwest.rs".into(),
        body: "http://docs.rs/reqwest".into(),
        user_id: 1,
    };
    let new_post: Post = reqwest::Client::new()
        .post("http://jsonplaceholder.typicode.com/posts")
        .json(&new_post)
        .send()
        .await?
        .json()
        .await?;

    println!("{:#?}", new_post);
    // Post {
    //     id: Some(
    //         101
    //     ),
    //     title: "Reqwest.rs",
    //     body: "https://docs.rs/reqwest",
    //     user_id: 1
    // }
    Ok(())
}
