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
        UploadPayload {
            video_uri: self.s3video_uri,
        }
    }
}
