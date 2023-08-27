-- Add down migration script here
ALTER TABLE
  videos
ADD
  COLUMN channel_id INTEGER REFERENCES channels(id);

UPDATE
  videos
SET
  channel_id = videos_channels.channel_id
FROM
  videos_channels
WHERE
  videos.id = videos_channels.video_id;

ALTER TABLE
  videos
ALTER COLUMN
  channel_id
SET
  NOT NULL;