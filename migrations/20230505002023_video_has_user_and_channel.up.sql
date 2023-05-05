-- Add up migration script here
ALTER TABLE videos ADD COLUMN user_id INTEGER NOT NULL REFERENCES users(id);
ALTER TABLE videos ADD COLUMN channel_id INTEGER NOT NULL REFERENCES channels(id);