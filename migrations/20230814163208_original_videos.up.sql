-- Add up migration script here
CREATE TABLE original_videos(
  id SERIAL PRIMARY KEY,
  url VARCHAR(255) NOT NULL,
  duration VARCHAR(255) NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);
--
INSERT INTO original_videos (url, duration)
SELECT v.original_url,
  v.original_duration
FROM (
    SELECT DISTINCT original_url,
      original_duration
    FROM videos
  ) AS v;
-- 
ALTER TABLE videos
ADD COLUMN original_video_id INTEGER NULL REFERENCES original_videos(id);
--
UPDATE videos
SET original_video_id = ov.id
FROM original_videos AS ov
WHERE videos.original_url = ov.url
  AND videos.original_duration = ov.duration;
--
ALTER TABLE videos DROP COLUMN original_url,
DROP COLUMN original_duration;
--
ALTER TABLE videos
ALTER COLUMN original_video_id
SET NOT NULL;