#[derive(Debug)]
pub struct UploadPayload {
    pub video_uri: String,
}

#[derive(Debug)]
pub enum PayloadType {
    BatukaVideoUpload(UploadPayload),
}
