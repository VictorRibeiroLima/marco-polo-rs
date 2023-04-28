use crate::internals::cloud::models::payload::UploadPayload;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct S3UploadPayload {
    #[serde(rename = "s3VideoURI")]
    pub s3video_uri: String,
}

impl Into<UploadPayload> for S3UploadPayload {
    fn into(self) -> UploadPayload {
        let video_id = self
            .s3video_uri
            .split('/')
            .filter(|x| uuid::Uuid::parse_str(x).is_ok())
            .next()
            .unwrap();

        let video_id = uuid::Uuid::parse_str(video_id).unwrap();

        UploadPayload {
            video_id,
            video_uri: self.s3video_uri,
        }
    }
}
