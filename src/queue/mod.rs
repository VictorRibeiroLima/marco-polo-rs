use actix_web::rt::Runtime;

use crate::internals::cloud::aws::AwsCloudService;

use self::worker::Worker;

mod worker;

pub fn init() {
    let rt = Runtime::new().unwrap();
    let queue_url = std::env::var("AWS_QUEUE_URL").expect("QUEUE_URL not found");
    let cloud_service = AwsCloudService::new(queue_url).unwrap();
    let worker = Worker { cloud_service };

    rt.block_on(async move {
        worker.handle_queue().await;
    });
}
