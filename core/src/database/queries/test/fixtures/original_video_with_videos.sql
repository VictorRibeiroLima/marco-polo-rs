--original video
INSERT INTO
  ORIGINAL_VIDEOS(id, url, duration)
VALUES
  (
    1,
    'https://www.youtube.com/watch?v=1234567890',
    '00:10:00'
  );

--user
INSERT INTO
  USERS (id, name, email, password, role)
VALUES
  (
    9999,
    'TestUser',
    'teste6666@gmail.com',
    '$2b$12$.jvb858VF4tanKNd11Vp4eDYyhg.KuFgOG8AhgJCvj/cJV47Sqtby',
    'USER'
  );

--channel
INSERT INTO
  channels (id, name, creator_id)
VALUES
  (9999, 'TestChannel', 9999);

--videos
----
CREATE
OR REPLACE FUNCTION populate_videos_for_original() RETURNS VOID AS $$ --function
DECLARE
  video_id_prefix TEXT := '806b5a48-f221-11ed-a05b-0242ac120';

video_id UUID;

BEGIN
  FOR i IN 1 ..20
  LOOP
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
    9999,
    1,
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
  (video_id, 9999);

END
LOOP
;

END;

$$ LANGUAGE plpgsql;

---
----
-- populate videos
SELECT
  populate_videos_for_original();