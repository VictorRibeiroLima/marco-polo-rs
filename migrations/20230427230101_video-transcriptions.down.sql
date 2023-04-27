-- Add down migration script here
ALTER TABLE videos_transcriptions DROP CONSTRAINT IF EXISTS fk_videos_transcriptions_video_id;
ALTER TABLE videos_transcriptions DROP CONSTRAINT IF EXISTS fk_videos_transcriptions_transcriber_id;
ALTER TABLE videos_transcriptions DROP CONSTRAINT IF EXISTS fk_videos_transcriptions_storage_id;
DROP INDEX IF EXISTS idx_videos_transcriptions_transcription_id;

DROP TABLE IF EXISTS videos_transcriptions;