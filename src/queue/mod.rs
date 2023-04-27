use actix_web::rt::Runtime;

use crate::internals::cloud::aws::{s3::S3Client, sqs::SQSClient};

use self::worker::handle_uploaded_file;

mod worker;

pub fn init() {
    let rt = Runtime::new().unwrap();
    let queue_client = SQSClient::new(std::env::var("AWS_QUEUE_URL").expect("QUEUE_URL not found"));
    let bucket_client = S3Client::new().unwrap();
    rt.block_on(async move {
        handle_uploaded_file(queue_client, bucket_client).await;
    });
}
