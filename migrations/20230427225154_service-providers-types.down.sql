-- Add down migration script here
ALTER TABLE service_providers_types DROP CONSTRAINT IF EXISTS fk_service_providers_types_service_provider_id;
ALTER TABLE service_providers_types DROP CONSTRAINT IF EXISTS fk_service_providers_types_type_id;
DROP TABLE IF EXISTS service_providers_types;