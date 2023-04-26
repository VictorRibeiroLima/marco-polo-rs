use std::thread;

use actix_web::rt::Runtime;
use infra::aws::payload::PayloadType;

mod api;
mod infra;

async fn test_queue<T, M>(client: T)
where
    T: infra::traits::QueueClient<M>,
    M: infra::traits::QueueMessage + std::fmt::Debug,
{
    loop {
        let message_result = client.receive_message().await.unwrap();
        let messages = match message_result {
            Some(messages) => messages,
            _ => {
                continue;
            }
        };

        for message in messages {
            let body = message.get_message();
            if body.is_empty() {
                continue;
            }

            let payload_type = match PayloadType::from_str(body.as_str()) {
                Ok(payload) => payload,
                Err(_) => {
                    continue;
                }
            };

            let payload = match payload_type {
                PayloadType::BatukaVideoUpload(payload) => payload,
            };

            println!("{:?}", payload.s3video_uri);

            let delete_result = client.delete_message(message).await; //TODO: see what to when delete fails

            match delete_result {
                Ok(_) => {}
                Err(e) => {
                    println!("{:?}", e);
                }
            }
        }
    }
}

fn thread_test() {
    let rt = Runtime::new().unwrap();
    let client = infra::aws::sqs::SQSClient::new(
        std::env::var("AWS_QUEUE_URL").expect("QUEUE_URL not found"),
    );
    rt.block_on(async move {
        test_queue(client).await;
    });
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::from_filename(".env").expect("Failed to load .env file");
    env_logger::init();
    thread::spawn(thread_test);
    api::init().await
}
