-- Add down migration script here
ALTER TABLE videos DROP COLUMN stage;
DROP TYPE videos_video_stages;