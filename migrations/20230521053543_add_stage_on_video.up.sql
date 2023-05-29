-- Add up migration script here
CREATE TYPE videos_video_stages AS ENUM ('DOWNLOADING','TRANSCRIBING','TRANSLATING','SUBTITLING','DONE');
ALTER TABLE videos ADD COLUMN stage videos_video_stages NOT NULL DEFAULT 'DOWNLOADING';
