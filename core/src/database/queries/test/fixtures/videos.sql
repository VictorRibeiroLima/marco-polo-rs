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
    '806b5a48-f221-11ed-a05b-0242ac120096',
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
    '806b5a48-f221-11ed-a05b-0242ac120096',
    666,
    'https://www.youtube.com/watch?v=1234567890'
  ),
  (
    '806b5a48-f221-11ed-a05b-0242ac120096',
    667,
    'https://www.youtube.com/watch?v=1234567890'
  );

-- create function
----
CREATE
OR REPLACE FUNCTION populate_videos() RETURNS VOID AS $$
DECLARE
  video_id_prefix TEXT := '806b5a48-f221-11ed-a05b-0242ac120';

video_id UUID;

BEGIN
  FOR i IN 1 ..20
  LOOP
  INSERT INTO
    ORIGINAL_VIDEOS(id, url, duration)
  VALUES
    (
      i,
      'https://www.youtube.com/watch?v=1234567890',
      '00:10:00'
    );

video_id := video_id_prefix || to_char(i, 'FM000');

INSERT INTO
  VIDEOS (
    id,
    title,
    description,
    user_id,
    original_video_id,
    start_time,
    end_time,
    tags,
    deleted_at
  )
VALUES
  (
    video_id,
    'Test Video ' || i,
    'This is a test video',
    666,
    i,
    '00:00:00',
    '00:10:00',
    'test',
    CASE
      WHEN i % 2 = 0 THEN '2021-09-01' :: timestamp
      ELSE NULL
    END
  );

INSERT INTO
  videos_channels (video_id, channel_id)
VALUES
  (video_id, 666),
  (video_id, 667);

END
LOOP
;

END;

$$ LANGUAGE plpgsql;

---
----
-- populate videos
SELECT
  populate_videos();