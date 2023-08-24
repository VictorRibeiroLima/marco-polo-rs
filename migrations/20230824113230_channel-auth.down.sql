ALTER TABLE
  channels
ADD
  csrf_token TEXT,
ADD
  refresh_token TEXT;

-- Update old columns using JSONB data
UPDATE
  channels
SET
  csrf_token = auth -> 'data' ->> 'csrf_token',
  refresh_token = auth -> 'data' ->> 'refresh_token';

-- Drop the JSONB column
ALTER TABLE
  channels DROP COLUMN auth;