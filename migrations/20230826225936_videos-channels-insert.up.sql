-- Add up migration script here
INSERT INTO
  videos_channels (video_id, channel_id, url)
SELECT
  id,
  channel_id,
  url
FROM
  videos;

ALTER TABLE
  videos DROP COLUMN channel_id;

ALTER TABLE
  videos DROP COLUMN url;