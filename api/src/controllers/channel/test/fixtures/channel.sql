INSERT INTO channels (id, name,created_at,updated_at) 
VALUES (1, 'Test Channel', '2022-01-01','2022-01-01');

INSERT INTO channels (id, name,created_at,updated_at,deleted_at) 
VALUES (2, 'Test Channel', '2022-01-01','2022-01-01',NOW()); 