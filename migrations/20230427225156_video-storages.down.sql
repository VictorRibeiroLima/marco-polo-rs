-- Add down migration script here
ALTER TABLE videos_storages DROP CONSTRAINT IF EXISTS fk_videos_storages_video_id;
ALTER TABLE videos_storages DROP CONSTRAINT IF EXISTS fk_videos_storages_storage_id;

DROP TABLE IF EXISTS videos_storages;
DROP TYPE IF EXISTS video_stage;
DROP TYPE IF EXISTS video_format;