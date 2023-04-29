use crate::internals::cloud::{
    models::payload::PayloadType,
    traits::{QueueClient, QueueMessage},
};
use async_trait::async_trait;
use rusoto_sqs::{
    ChangeMessageVisibilityRequest, DeleteMessageRequest, Message, ReceiveMessageRequest, Sqs,
    SqsClient,
};
use serde_json::Value;

use super::payload::{S3SrtTranscriptionPayload, S3UploadPayload};

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

    fn to_payload(
        &self,
    ) -> Result<crate::internals::cloud::models::payload::PayloadType, Box<dyn std::error::Error>>
    {
        let body = match &self.body {
            Some(body) => body,
            None => return Err("No body found".into()),
        };
        let v: Value = serde_json::from_str(body)?;
        let type_field = match v["type"].as_str() {
            Some(type_field) => type_field,
            None => return Err("No type field".into()),
        };
        if v["payload"].is_null() {
            return Err("No payload field".into());
        }
        let payload = v["payload"].to_string();
        match type_field {
            "BatukaVideoUpload" => {
                let payload: S3UploadPayload = serde_json::from_str(&payload)?;
                return Ok(PayloadType::BatukaVideoUpload(payload.into()));
            }
            "BatukaSrtTranscriptionUpload" => {
                let payload: S3SrtTranscriptionPayload = serde_json::from_str(&payload)?;
                return Ok(PayloadType::BatukaSrtTranscriptionUpload(payload.into()));
            }
            _ => Err("Invalid type field".into()),
        }
    }
}

#[async_trait]
impl QueueClient for SQSClient {
    type M = Message;
    async fn receive_message(&self) -> Result<Option<Vec<Self::M>>, Box<dyn std::error::Error>> {
        let request = ReceiveMessageRequest {
            queue_url: self.queue_url.clone(),
            max_number_of_messages: Some(10),
            wait_time_seconds: Some(20),
            ..Default::default()
        };

        let output = self.client.receive_message(request).await?;
        return Ok(output.messages);
    }

    async fn send_message(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    async fn delete_message(&self, message: Self::M) -> Result<(), Box<dyn std::error::Error>> {
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

    async fn change_message_visibility(
        &self,
        message: &Self::M,
        visibility_timeout: usize,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let receipt_handle = match message.receipt_handle.as_ref() {
            Some(receipt_handle) => receipt_handle.to_string(),
            None => {
                return Err("No receipt handle found".into());
            }
        };
        let request = ChangeMessageVisibilityRequest {
            queue_url: self.queue_url.clone(),
            receipt_handle,
            visibility_timeout: visibility_timeout as i64,
            ..Default::default()
        };

        self.client.change_message_visibility(request).await?;
        return Ok(());
    }
}
