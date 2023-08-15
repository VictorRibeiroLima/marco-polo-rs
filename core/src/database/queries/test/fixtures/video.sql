INSERT INTO USERS (id, name, email, password, role)
VALUES (
    666,
    'TestUser',
    'teste@gmail.com',
    '$2b$12$.jvb858VF4tanKNd11Vp4eDYyhg.KuFgOG8AhgJCvj/cJV47Sqtby',
    'USER'
  );
--99020711Aa@
INSERT INTO channels (id, name, creator_id)
VALUES (666, 'TestChannel', 666);
--original video
INSERT INTO ORIGINAL_VIDEOS(id, url, duration)
VALUES (
    666,
    'https://www.youtube.com/watch?v=1234567890',
    '00:10:00'
  );
-- with url 
INSERT INTO VIDEOS (
    id,
    title,
    url,
    description,
    user_id,
    channel_id,
    original_video_id,
    start_time,
    end_time,
    tags
  )
VALUES (
    '806b5a48-f221-11ed-a05b-0242ac120096',
    'Space Tourism Test',
    'https://www.youtube.com/watch?v=1234567890',
    'This is a test video about space tourism',
    666,
    666,
    666,
    '00:00:00',
    '00:10:00',
    'spacetourism'
  );