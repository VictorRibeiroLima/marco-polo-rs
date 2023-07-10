INSERT INTO users (id,name,email,password) VALUES (999,'TestUser','test@test.com','TestPassword');

INSERT INTO channels (id, name,created_at,updated_at,creator_id) 
VALUES (1, 'Test Channel', '2022-01-01','2022-01-01',999);

INSERT INTO channels (id, name,created_at,updated_at,deleted_at,creator_id) 
VALUES (2, 'Test Channel', '2022-01-01','2022-01-01',NOW(),999); 