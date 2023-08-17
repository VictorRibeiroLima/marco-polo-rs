ALTER TABLE videos
ADD COLUMN original_url VARCHAR(255) NULL,
  ADD COLUMN original_duration VARCHAR(255) NULL;
--

UPDATE videos
SET original_url = ov.url,
  original_duration = ov.duration
FROM original_videos AS ov
WHERE videos.original_video_id = ov.id;
--

ALTER TABLE videos DROP COLUMN original_video_id;
--
ALTER TABLE videos
ALTER COLUMN original_url
SET NOT NULL;
--
DROP TABLE original_videos;