use futures_util::StreamExt;
use rskafka::client::{
    consumer::{StartOffset, StreamConsumerBuilder},
    partition::UnknownTopicHandling,
    ClientBuilder,
};
use std::sync::Arc;

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
struct Order {
    order_id: i32,
    product_id: i32,
    quantity: i32,
    amount: f32,
    shipping: f32,
    tax: f32,
    shipping_address: String,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let host = "127.0.0.1";
    let port = 56839;
    let topic = "order";
    let connection = format!("{}:{}", host, port);
    let timeout = tokio::time::timeout(
        std::time::Duration::from_secs(3),
        ClientBuilder::new(vec![connection]).build(),
    )
    .await;
    let client = if let Ok(client) = timeout {
        client?
    } else {
        panic!("Cannot connect Kafka in 3s.");
    };

    let partition_client = Arc::new(
        client
            .partition_client(
                topic,
                0, // partition
                UnknownTopicHandling::Retry,
            )
            .await?,
    );
    let mut stream = StreamConsumerBuilder::new(Arc::clone(&partition_client), StartOffset::Latest)
        .with_max_wait_ms(500)
        .build();
    // use loop to listen incoming records.
    loop {
        let (mut record, _high_watermark) = stream.next().await.expect("No data")?;
        if let Some(incoming_data) = record.record.value.take() {
            println!("get a record");
            let s = std::str::from_utf8(&incoming_data)?;
            let order: Order = serde_json::from_str(String::from(s).as_str())?;
            println!("{:?}", &order);
        }
    }
}
