-- Add up migration script here
CREATE TABLE IF NOT EXISTS videos_subtitlings (
    video_id uuid NOT NULL,
    subtitler_id integer NOT NULL,
    subtitling_id varchar(255),
    created_at timestamp NOT NULL DEFAULT now(),
    updated_at timestamp NOT NULL DEFAULT now(),
    deleted_at timestamp,
    CONSTRAINT pk_videos_subtitlings PRIMARY KEY (video_id)
);

ALTER TABLE videos_subtitlings ADD CONSTRAINT fk_videos_subtitlings_video_id FOREIGN KEY (video_id) REFERENCES videos(id);
ALTER TABLE videos_subtitlings ADD CONSTRAINT fk_videos_subtitlings_subtitler_id FOREIGN KEY (subtitler_id) REFERENCES service_providers(id);

CREATE INDEX idx_videos_subtitlings_subtitling_id ON videos_subtitlings (subtitling_id);