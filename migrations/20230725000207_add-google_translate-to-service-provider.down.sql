-- Add down migration script here
DELETE FROM service_providers WHERE name = 'google_translate';