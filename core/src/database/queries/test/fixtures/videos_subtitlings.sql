INSERT INTO
  USERS (id, name, email, password, role)
VALUES
  (
    666,
    'TestUser',
    'teste@gmail.com',
    '$2b$12$.jvb858VF4tanKNd11Vp4eDYyhg.KuFgOG8AhgJCvj/cJV47Sqtby',
    'USER'
  );

--99020711Aa@
INSERT INTO
  channels (id, name, creator_id, platform)
VALUES
  (666, 'TestChannel', 666, 'YOUTUBE'),
  (667, 'TestChannel2', 666, 'TIKTOK');

--original video
INSERT INTO
  ORIGINAL_VIDEOS(id, url, duration)
VALUES
  (
    666,
    'https://www.youtube.com/watch?v=1234567890',
    '00:10:00'
  );

-- with url 
INSERT INTO
  VIDEOS (
    id,
    title,
    description,
    user_id,
    original_video_id,
    start_time,
    end_time,
    tags
  )
VALUES
  (
    '806b57d2-f221-11ed-a05b-0242ac120003',
    'Space Tourism Test',
    'This is a test video about space tourism',
    666,
    666,
    '00:00:00',
    '00:10:00',
    'spacetourism'
  );

INSERT INTO
  videos_channels (video_id, channel_id, url)
VALUES
  (
    '806b57d2-f221-11ed-a05b-0242ac120003',
    666,
    'https://www.youtube.com/watch?v=1234567890'
  ),
  (
    '806b57d2-f221-11ed-a05b-0242ac120003',
    667,
    'https://www.youtube.com/watch?v=1234567890'
  );

INSERT INTO
  service_providers (id, name)
VALUES
  (123, 'Nome do Provedor');

INSERT INTO
  VIDEOS_SUBTITLINGS (video_id, subtitler_id, subtitling_id)
VALUES
  (
    '806b57d2-f221-11ed-a05b-0242ac120003',
    123,
    'This is a test video about Elon Musk'
  );