use lambda_runtime::{Error, Context, handler_fn};
use aws_lambda_events::event::s3::S3Event;
use serde_json::json;
use aws_sdk_s3::Client;
use aws_config::BehaviorVersion;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = handler_fn(my_handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn my_handler(event: S3Event, _context: Context) -> Result<serde_json::Value, Error> {
    let client = Client::new(&aws_config::load_defaults(BehaviorVersion::latest()).await);

    // Assuming you have a way to get the bucket name from the event
    let bucket_name = "photo-rag";

    // Count the number of items in the bucket
    let num_items = count_items_in_bucket(&client, bucket_name).await?;

    // Construct and return the response
    let response = json!({
        "message": "Number of items in the bucket",
        "bucket": bucket_name,
        "count": num_items
    });

    Ok(response)
}

async fn count_items_in_bucket(client: &Client, bucket_name: &str) -> Result<usize, Error> {
    let resp = client.list_objects_v2().bucket(bucket_name).send().await?;
    let contents = resp.contents().len();
    Ok(contents)
}