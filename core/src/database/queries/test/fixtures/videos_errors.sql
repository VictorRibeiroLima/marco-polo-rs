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

INSERT INTO
  channels (id, name, creator_id)
VALUES
  (666, 'TestChannel', 666);

INSERT INTO
  VIDEOS (
    id,
    title,
    description,
    url,
    user_id,
    channel_id,
    error
  )
VALUES
  (
    '806b5a48-f221-11ed-a05b-0242ac120096',
    'Space Tourism Test',
    'This is a test video about space tourism',
    'https://www.youtube.com/watch?v=1234567890',
    666,
    666,
    true
  );

INSERT INTO
  VIDEOS_ERRORS (video_id, error, stage)
VALUES
  (
    '806b5a48-f221-11ed-a05b-0242ac120096',
    'Error message',
    'DOWNLOADING'
  );