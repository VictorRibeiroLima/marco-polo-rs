-- Add down migration script here
ALTER TABLE
  channels DROP COLUMN platform;

DROP TYPE video_platforms;