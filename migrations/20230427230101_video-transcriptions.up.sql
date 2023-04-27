-- Add up migration script here
CREATE TABLE IF NOT EXISTS videos_transcriptions (
    video_id uuid PRIMARY KEY,
    transcriber_id integer NOT NULL,
    transcription_id varchar(255) NOT NULL,
    storage_id integer,
    path varchar(255),
    created_at timestamp NOT NULL DEFAULT now(),
    updated_at timestamp NOT NULL DEFAULT now(),
    deleted_at timestamp
);

ALTER TABLE videos_transcriptions ADD CONSTRAINT fk_videos_transcriptions_video_id FOREIGN KEY (video_id) REFERENCES videos(id);
ALTER TABLE videos_transcriptions ADD CONSTRAINT fk_videos_transcriptions_transcriber_id FOREIGN KEY (transcriber_id) REFERENCES service_providers(id);
ALTER TABLE videos_transcriptions ADD CONSTRAINT fk_videos_transcriptions_storage_id FOREIGN KEY (storage_id) REFERENCES service_providers(id);

CREATE INDEX idx_videos_transcriptions_transcription_id ON videos_transcriptions (transcription_id);