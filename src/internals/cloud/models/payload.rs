use uuid::Uuid;

#[derive(Debug)]
pub struct UploadPayload {
    pub video_uri: String,
    pub video_id: Uuid,
}

#[derive(Debug)]
pub enum PayloadType {
    BatukaVideoUpload(UploadPayload),
}
