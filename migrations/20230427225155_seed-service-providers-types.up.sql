-- Add up migration script here
INSERT INTO service_providers_types (service_provider_id, service_type_id) VALUES (1, 4); -- LOCAL - SUBTITLING
INSERT INTO service_providers_types (service_provider_id, service_type_id) VALUES (2, 1); -- AWS - STORAGE
INSERT INTO service_providers_types (service_provider_id, service_type_id) VALUES (3, 2); -- AssemblyAI - TRANSCRIPTION
INSERT INTO service_providers_types (service_provider_id, service_type_id) VALUES (4, 3); -- DeepL - TRANSLATION