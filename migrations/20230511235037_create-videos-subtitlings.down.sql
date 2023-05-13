-- Add down migration script here
ALTER TABLE videos_subtitlings DROP CONSTRAINT IF EXISTS fk_videos_subtitlings_video_id;
ALTER TABLE videos_subtitlings DROP CONSTRAINT IF EXISTS fk_videos_subtitlings_subtitler_id;
DROP INDEX IF EXISTS idx_videos_subtitlings_subtitling_id;

DROP TABLE IF EXISTS videos_subtitlings;