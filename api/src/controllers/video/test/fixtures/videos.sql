INSERT INTO USERS (id, name, email, password, role) 
VALUES (123, 'TestUser', 'teste123@gmail.com', '$2b$12$.jvb858VF4tanKNd11Vp4eDYyhg.KuFgOG8AhgJCvj/cJV47Sqtby', 'USER'); --99020711Aa@

INSERT INTO channels (id,name,creator_id) VALUES (666,'TestChannel',666);

INSERT INTO VIDEOS (id, title, description, user_id, channel_id, url, language, created_at, updated_at, uploaded_at) 
VALUES (
    '806b57d2-f221-11ed-a05b-0242ac120003',
    'Elon Musk Test',
    'This is a test video about Elon Musk',
    123,
    666,
    'https://video.com',
    'English',
    '2022-01-01',
    '2022-01-01',
    '2022-01-01'
);