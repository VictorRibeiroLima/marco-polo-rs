-- Add up migration script here
INSERT INTO
  videos_channels (video_id, channel_id)
SELECT
  id,
  channel_id
FROM
  videos;

ALTER TABLE
  videos DROP COLUMN channel_id;