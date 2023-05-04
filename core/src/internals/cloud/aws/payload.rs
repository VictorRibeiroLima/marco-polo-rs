use crate::internals::cloud::models::payload::{SrtPayload, VideoPayload};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct S3UploadPayload {
    #[serde(rename = "s3VideoURI")]
    pub s3video_uri: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct S3SrtPayload {
    #[serde(rename = "s3SrtURI")]
    pub s3srt_uri: String,
}

impl Into<VideoPayload> for S3UploadPayload {
    fn into(self) -> VideoPayload {
        let video_id = self
            .s3video_uri
            .split('/')
            .last()
            .unwrap()
            .split('.')
            .next()
            .unwrap();

        let video_id = uuid::Uuid::parse_str(video_id).unwrap();

        VideoPayload {
            video_id,
            video_uri: self.s3video_uri,
        }
    }
}

impl Into<SrtPayload> for S3SrtPayload {
    fn into(self) -> SrtPayload {
        let video_id = self
            .s3srt_uri
            .split('/')
            .last()
            .unwrap()
            .split('.')
            .next()
            .unwrap();

        let video_id = uuid::Uuid::parse_str(video_id).unwrap();

        SrtPayload {
            video_id,
            srt_uri: self.s3srt_uri,
        }
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_into_upload_payload() {
        let uuid = uuid::Uuid::new_v4();
        let uri = format!("videos/{}.mkv", uuid);
        let s3_upload_payload = super::S3UploadPayload {
            s3video_uri: uri.clone(),
        };

        let upload_payload: super::VideoPayload = s3_upload_payload.into();

        assert_eq!(upload_payload.video_uri, uri);
        assert_eq!(upload_payload.video_id, uuid);
    }
}
