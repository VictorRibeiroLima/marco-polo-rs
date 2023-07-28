-- Add up migration script here
ALTER TABLE videos ADD COLUMN error BOOLEAN NOT NULL DEFAULT FALSE;


CREATE TABLE videos_errors (
  id SERIAL PRIMARY KEY,
  video_id UUID NOT NULL REFERENCES videos(id),
  error TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  stage videos_video_stages NOT NULL
);