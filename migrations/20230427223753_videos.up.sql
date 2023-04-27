-- Add up migration script here
CREATE TABLE IF NOT EXISTS videos (
  id uuid PRIMARY KEY,
  title varchar(255),
  description text,
  url varchar(255),
  language varchar(255) NOT NULL DEFAULT 'en',
  created_at timestamp NOT NULL DEFAULT now(),
  updated_at timestamp NOT NULL DEFAULT now(),
  deleted_at timestamp,
  uploaded_at timestamp
);