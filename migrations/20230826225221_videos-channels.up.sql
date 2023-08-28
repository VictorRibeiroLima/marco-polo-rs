-- Add up migration script here
CREATE TABLE videos_channels (
  video_id uuid NOT NULL,
  channel_id integer NOT NULL,
  uploaded boolean NOT NULL DEFAULT false,
  error boolean NOT NULL DEFAULT false,
  url varchar(255),
  created_at timestamp NOT NULL DEFAULT now(),
  updated_at timestamp NOT NULL DEFAULT now(),
  uploaded_at timestamp,
  PRIMARY KEY (video_id, channel_id)
);

ALTER TABLE
  videos_channels
ADD
  CONSTRAINT fk_videos_channels_video_id FOREIGN KEY (video_id) REFERENCES videos (id) ON DELETE CASCADE;

ALTER TABLE
  videos_channels
ADD
  CONSTRAINT fk_videos_channels_channel_id FOREIGN KEY (channel_id) REFERENCES channels (id) ON DELETE CASCADE;