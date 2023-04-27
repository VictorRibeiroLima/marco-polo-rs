-- Add down migration script here
ALTER TABLE videos_translations DROP CONSTRAINT IF EXISTS fk_videos_translations_video_id;
ALTER TABLE videos_translations DROP CONSTRAINT IF EXISTS fk_videos_translations_translator_id;
ALTER TABLE videos_translations DROP CONSTRAINT IF EXISTS fk_videos_translations_storage_id;

DROP INDEX IF EXISTS idx_videos_translations_translation_id;
DROP TABLE IF EXISTS video_translations;