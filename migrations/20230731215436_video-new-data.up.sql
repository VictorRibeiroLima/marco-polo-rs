-- Add up migration script here
ALTER TABLE
  videos
ADD
  COLUMN original_url VARCHAR(255);

UPDATE
  videos
SET
  original_url = 'migration';

ALTER TABLE
  videos
ALTER COLUMN
  original_url
SET
  NOT NULL;

ALTER TABLE
  videos
ADD
  COLUMN original_duration VARCHAR(255);

ALTER TABLE
  videos
ADD
  COLUMN start_time VARCHAR(20);

UPDATE
  videos
SET
  start_time = 'migration';

ALTER TABLE
  videos
ALTER COLUMN
  start_time
SET
  NOT NULL;

ALTER TABLE
  videos
ADD
  COLUMN end_time VARCHAR(20);

UPDATE
  videos
SET
  end_time = 'migration';

ALTER TABLE
  videos
ADD
  COLUMN tags VARCHAR(255);