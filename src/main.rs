use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use async_lambda_s3::{client, count_items_in_bucket};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Request {
    name: String,
}

#[derive(Serialize)]
struct Response {
    req_id: String,
    msg: String,
    count: usize,
}

async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    let s3_client = client().await?;
    let bucket_name = "photo-rag"; // Specify the bucket name
    let item_count = count_items_in_bucket(&s3_client, bucket_name).await?;
    
    let resp = Response {
        req_id: event.context.request_id,
        count: item_count,
        msg: format!("Item count in bucket '{}': {}", bucket_name, item_count),
    };

    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}