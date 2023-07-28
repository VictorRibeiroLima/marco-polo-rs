-- Add down migration script here
CREATE TYPE videos_video_stages_temp AS ENUM ('DOWNLOADING','TRANSCRIBING','TRANSLATING','SUBTITLING','DONE');


ALTER TABLE videos ALTER COLUMN stage DROP DEFAULT;

ALTER TABLE videos ALTER COLUMN stage TYPE videos_video_stages_temp USING stage::text::videos_video_stages_temp;
ALTER TABLE videos_errors ALTER COLUMN stage TYPE videos_video_stages_temp USING stage::text::videos_video_stages_temp;

DROP TYPE videos_video_stages;

ALTER TYPE videos_video_stages_temp RENAME TO videos_video_stages;

ALTER TABLE videos ALTER COLUMN stage SET DEFAULT 'DOWNLOADING';