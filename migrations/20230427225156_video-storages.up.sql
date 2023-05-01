-- Add up migration script here
CREATE TYPE video_stage AS ENUM ('RAW', 'PROCESSED');
CREATE TYPE video_format AS ENUM ('MP4', 'AVI', 'MOV');

CREATE TABLE IF NOT EXISTS videos_storages (
  id serial PRIMARY KEY,
  video_id uuid NOT NULL,
  storage_id integer NOT NULL,
  stage video_stage NOT NULL,
  format video_format NOT NULL,
  video_path varchar(255) NOT NULL,
  created_at timestamp NOT NULL DEFAULT now(),
  updated_at timestamp NOT NULL DEFAULT now(),
  deleted_at timestamp
);

ALTER TABLE videos_storages ADD CONSTRAINT fk_videos_storages_video_id FOREIGN KEY (video_id) REFERENCES videos(id);
ALTER TABLE videos_storages ADD CONSTRAINT fk_videos_storages_storage_id FOREIGN KEY (storage_id) REFERENCES service_providers(id);