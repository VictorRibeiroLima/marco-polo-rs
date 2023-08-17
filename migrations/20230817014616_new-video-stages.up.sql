-- Add up migration script here
ALTER TYPE videos_video_stages
ADD VALUE 'CUTTING';
ALTER TYPE videos_video_stages
ADD VALUE 'RAW_UPLOADING';