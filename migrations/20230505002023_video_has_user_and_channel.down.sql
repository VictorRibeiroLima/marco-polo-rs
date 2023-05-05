-- Add down migration script here
ALTER TABLE videos DROP COLUMN user_id;
ALTER TABLE videos DROP COLUMN channel_id;