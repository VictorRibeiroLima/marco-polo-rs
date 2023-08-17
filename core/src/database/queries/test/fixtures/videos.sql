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
--fixed value video
INSERT INTO ORIGINAL_VIDEOS(id, url, duration)
VALUES (
    1000,
    'https://www.youtube.com/watch?v=1234567890',
    '00:10:00'
  );
INSERT INTO VIDEOS (
    id,
    title,
    description,
    user_id,
    channel_id,
    original_video_id,
    start_time,
    end_time,
    url,
    tags
  )
VALUES(
    '806b5a48-f221-11ed-a05b-0242ac120096',
    'Test Video 1000',
    'This is a test video',
    666,
    666,
    1000,
    '00:00:00',
    '00:10:00',
    'https://www.youtube.com/watch?v=1234567890',
    'test'
  );
-- create function
----
CREATE OR REPLACE FUNCTION populate_videos() RETURNS VOID AS $$ --function
DECLARE --declaration
  video_id_prefix TEXT := '806b5a48-f221-11ed-a05b-0242ac120';
video_id UUID;
BEGIN --begin
FOR i IN 1..20 LOOP
INSERT INTO ORIGINAL_VIDEOS(id, url, duration)
VALUES (
    i,
    'https://www.youtube.com/watch?v=1234567890',
    '00:10:00'
  );
video_id := video_id_prefix || to_char(i, 'FM000');
INSERT INTO VIDEOS (
    id,
    title,
    description,
    user_id,
    channel_id,
    original_video_id,
    start_time,
    end_time,
    tags,
    deleted_at
  )
VALUES(
    video_id,
    'Test Video ' || i,
    'This is a test video',
    666,
    666,
    i,
    '00:00:00',
    '00:10:00',
    'test',
    CASE
      WHEN i % 2 = 0 THEN '2021-09-01'::timestamp
      ELSE NULL
    END
  );
END LOOP;
END;
$$ LANGUAGE plpgsql;
---
----
-- populate videos
SELECT populate_videos();