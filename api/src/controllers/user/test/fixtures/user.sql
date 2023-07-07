INSERT INTO USERS (id, name, email, password, role,created_at,updated_at) 
VALUES (666, 'TestUser', 'teste@gmail.com', '$2b$12$.jvb858VF4tanKNd11Vp4eDYyhg.KuFgOG8AhgJCvj/cJV47Sqtby', 'USER','2022-01-01','2022-01-01'); --99020711Aa@

INSERT INTO USERS (id, name, email, password, role, deleted_at) 
VALUES (667, 'TestUser', 'teste1@gmail.com', '$2b$12$.jvb858VF4tanKNd11Vp4eDYyhg.KuFgOG8AhgJCvj/cJV47Sqtby', 'USER', NOW()); --99020711Aa@