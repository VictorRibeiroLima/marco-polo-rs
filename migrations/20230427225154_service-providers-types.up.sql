-- Add up migration script here
CREATE TABLE service_providers_types(
  service_provider_id INTEGER NOT NULL,
  service_type_id INTEGER NOT NULL,
  PRIMARY KEY (service_provider_id, service_type_id)
);

ALTER TABLE service_providers_types ADD CONSTRAINT fk_service_providers_types_service_provider_id FOREIGN KEY (service_provider_id) REFERENCES service_providers (id);
ALTER TABLE service_providers_types ADD CONSTRAINT fk_service_providers_types_type_id FOREIGN KEY (service_type_id) REFERENCES service_types (id);