-- Add up migration script here
ALTER TABLE videos
ALTER COLUMN tags TYPE text;