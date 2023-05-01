use uuid::Uuid;

#[derive(Debug)]
pub struct VideoPayload {
    pub video_uri: String,
    pub video_id: Uuid,
}

#[derive(Debug)]
pub struct SrtPayload {
    pub video_id: Uuid,
    pub srt_uri: String,
}

#[derive(Debug)]
pub enum PayloadType {
    BatukaVideoRawUpload(VideoPayload),
    BatukaVideoProcessedUpload(VideoPayload),
    BatukaSrtTranscriptionUpload(SrtPayload),
    BatukaSrtTranslationUpload(SrtPayload),
}
