-- Add down migration script here
ALTER TABLE channels DROP CONSTRAINT fk_channels_creator_id;
ALTER TABLE channels DROP COLUMN creator_id;