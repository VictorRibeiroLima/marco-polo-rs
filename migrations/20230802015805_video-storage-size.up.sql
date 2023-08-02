-- Add up migration script here
ALTER TABLE
  videos_storages
ADD
  COLUMN size bigint;

UPDATE
  videos_storages
SET
  size = 0;

ALTER TABLE
  videos_storages
ALTER COLUMN
  size
SET
  NOT NULL;