-- Add up migration script here
CREATE TABLE IF NOT EXISTS videos_translations (
    video_id UUID PRIMARY KEY,
    translator_id integer NOT NULL,
    translation_id varchar(255) NOT NULL,
    storage_id integer,
    path varchar(255),
    language VARCHAR(255) NOT NULL DEFAULT 'pt-br',
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMP
);

ALTER TABLE videos_translations ADD CONSTRAINT fk_videos_translations_video_id FOREIGN KEY (video_id) REFERENCES videos(id);
ALTER TABLE videos_translations ADD CONSTRAINT fk_videos_translations_translator_id FOREIGN KEY (translator_id) REFERENCES service_providers(id);
ALTER TABLE videos_translations ADD CONSTRAINT fk_videos_translations_storage_id FOREIGN KEY (storage_id) REFERENCES service_providers(id);


CREATE INDEX idx_videos_translations_translation_id ON videos_translations (translation_id);