-- Add down migration script here
ALTER TABLE
  videos DROP COLUMN original_url;

ALTER TABLE
  videos DROP COLUMN original_duration;

ALTER TABLE
  videos DROP COLUMN start_time;

ALTER TABLE
  videos DROP COLUMN end_time;

ALTER TABLE
  videos DROP COLUMN tags;