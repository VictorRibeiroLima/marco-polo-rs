-- Add down migration script here
ALTER TABLE videos DROP COLUMN error;

DROP TABLE video_errors;