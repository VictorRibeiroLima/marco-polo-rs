-- Add up migration script here
ALTER TABLE users
ADD COLUMN forgot_token VARCHAR(255) NULL;
ALTER TABLE users
ADD COLUMN forgot_token_expires_at TIMESTAMP NULL;