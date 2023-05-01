-- Add up migration script here

CREATE TABLE IF NOT EXISTS service_types (
  id serial PRIMARY KEY,
  name varchar(255) NOT NULL,
  created_at timestamp NOT NULL DEFAULT now(),
  updated_at timestamp NOT NULL DEFAULT now(),
  deleted_at timestamp
);