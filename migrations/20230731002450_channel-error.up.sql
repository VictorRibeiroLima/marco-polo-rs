-- Add up migration script here
ALTER TABLE
  channels
ADD
  COLUMN error BOOLEAN NOT NULL DEFAULT FALSE;