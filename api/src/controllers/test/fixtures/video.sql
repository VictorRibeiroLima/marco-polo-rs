INSERT INTO users (id,name,email,password) VALUES (999,'TestUser','test@test.com','TestPassword');

INSERT INTO channels (id, name,created_at,updated_at,creator_id) 
VALUES (1, 'Test Channel', '2022-01-01','2022-01-01',999);

INSERT INTO videos (id, title, description, user_id, channel_id, url, deleted_at, language)
VALUES ('2c20e6d2-7bce-47b7-b02d-7f45fb106df5', 'Travel Vlog', 'Join me on my adventures around the world', 999, 1, 'https://video.com/travel', NOW(), 'English');
