// LIB.RS
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{Client, Error};

// Function to create a new AWS S3 client
pub async fn client() -> Result<Client, Error> {
    let region_provider = RegionProviderChain::first_try(None)
        .or_default_provider()
        .or_else("us-east-1");
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);
    Ok(client)
}

// Function to count the number of items in a specific S3 bucket
pub async fn count_items_in_bucket(client: &Client, bucket_name: &str) -> Result<usize, Error> {
    let resp = client.list_objects_v2().bucket(bucket_name).send().await?;
    let contents = resp.contents().unwrap_or_default();
    Ok(contents.len())
}