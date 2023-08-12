-- Add down migration script here
ALTER TABLE users DROP COLUMN forgot_token;
ALTER TABLE users DROP COLUMN forgot_token_expires_at;