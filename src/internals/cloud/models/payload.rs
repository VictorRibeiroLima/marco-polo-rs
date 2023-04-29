use uuid::Uuid;

#[derive(Debug)]
pub struct UploadPayload {
    pub video_uri: String,
    pub video_id: Uuid,
}

#[derive(Debug)]
pub struct SrtTranscriptionPayload {
    pub video_id: Uuid,
    pub srt_uri: String,
}

#[derive(Debug)]
pub enum PayloadType {
    BatukaVideoUpload(UploadPayload),
    BatukaSrtTranscriptionUpload(SrtTranscriptionPayload),
}
