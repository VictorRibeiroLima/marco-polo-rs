-- Add up migration script here
ALTER TABLE
  channels
ADD
  auth jsonb NOT NULL DEFAULT '{}' :: jsonb;

UPDATE
  channels
SET
  auth = jsonb_build_object(
    'type',
    'OAUTH2',
    'data',
    jsonb_build_object(
      'csrf_token',
      csrf_token,
      'refresh_token',
      refresh_token
    )
  );

ALTER TABLE
  channels DROP COLUMN csrf_token,
  DROP COLUMN refresh_token;