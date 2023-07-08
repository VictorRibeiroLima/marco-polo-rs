-- Add up migration script here
ALTER TABLE channels ADD COLUMN creator_id INTEGER;
ALTER TABLE channels ADD CONSTRAINT fk_channels_creator_id FOREIGN KEY (creator_id) REFERENCES users (id);
UPDATE channels SET creator_id = 1;
ALTER TABLE channels ALTER COLUMN creator_id SET NOT NULL;