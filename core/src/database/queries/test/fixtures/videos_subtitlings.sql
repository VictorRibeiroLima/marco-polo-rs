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
  channels (id, name, creator_id)
VALUES
  (666, 'TestChannel', 666);

INSERT INTO
  VIDEOS (
    id,
    title,
    description,
    user_id,
    channel_id,
    original_url,
    original_end_time,
    start_time,
    end_time,
    tags
  )
VALUES
  (
    '806b57d2-f221-11ed-a05b-0242ac120003',
    'Elon Musk Test',
    'This is a test video about Elon Musk',
    666,
    666,
    'https://www.youtube.com/watch?v=ZUdHXRoqXoE',
    '00:10:00',
    '00:00:00',
    '00:10:00',
    'elon;musk'
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