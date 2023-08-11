-- Add down migration script here
ALTER TABLE videos
ALTER COLUMN tags TYPE varchar(255);