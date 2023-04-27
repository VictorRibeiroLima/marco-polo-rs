-- Add up migration script here
CREATE TYPE service_provider_type AS ENUM ('STORAGE', 'TRANSCRIPTION','TRANSLATION');

CREATE TABLE IF NOT EXISTS service_providers (
  id serial PRIMARY KEY,
  name varchar(255) NOT NULL,
  type service_provider_type NOT NULL,
  created_at timestamp NOT NULL DEFAULT now(),
  updated_at timestamp NOT NULL DEFAULT now(),
  deleted_at timestamp
);