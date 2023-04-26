use crate::infra::traits::{QueueClient, QueueMessage};
use async_trait::async_trait;
use rusoto_sqs::{DeleteMessageRequest, Message, ReceiveMessageRequest, Sqs, SqsClient};

pub struct SQSClient {
    client: SqsClient,
    queue_url: String,
}

impl SQSClient {
    pub fn new(queue_url: String) -> Self {
        let region = rusoto_core::Region::SaEast1;
        let client = SqsClient::new(region);
        SQSClient { client, queue_url }
    }
}

impl QueueMessage for Message {
    fn get_handle(&self) -> String {
        match &self.receipt_handle {
            Some(handle) => handle.clone(),
            None => String::new(),
        }
    }

    fn get_message(&self) -> String {
        match &self.body {
            Some(body) => body.clone(),
            None => String::new(),
        }
    }
}

#[async_trait]
impl QueueClient<Message> for SQSClient {
    async fn receive_message(&self) -> Result<Option<Vec<Message>>, Box<dyn std::error::Error>> {
        let request = ReceiveMessageRequest {
            queue_url: self.queue_url.clone(),
            max_number_of_messages: Some(10),
            wait_time_seconds: Some(20),
            ..Default::default()
        };

        let output = self.client.receive_message(request).await?;
        if let Some(messages) = output.messages {
            return Ok(Some(messages));
        }
        return Ok(None);
    }

    async fn send_message(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    async fn delete_message(&self, message: Message) -> Result<(), Box<dyn std::error::Error>> {
        let receipt_handle = match message.receipt_handle {
            Some(receipt_handle) => receipt_handle,
            None => {
                return Err("No receipt handle found".into());
            }
        };
        let delete_request = DeleteMessageRequest {
            queue_url: self.queue_url.clone(),
            receipt_handle,
        };

        let result = self.client.delete_message(delete_request).await;
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),
        }
    }
}
