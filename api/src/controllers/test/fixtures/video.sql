INSERT INTO users (id, name, email, password)
VALUES (999, 'TestUser', 'test@test.com', 'TestPassword');
INSERT INTO channels (id, name, created_at, updated_at, creator_id)
VALUES (
    1,
    'Test Channel',
    '2022-01-01',
    '2022-01-01',
    999
  );
--original video
INSERT INTO ORIGINAL_VIDEOS(id, url, duration)
VALUES (
    999,
    'https://www.youtube.com/watch?v=1234567890',
    '00:10:00'
  );
INSERT INTO videos (
    id,
    title,
    description,
    user_id,
    channel_id,
    url,
    deleted_at,
    language,
    original_video_id,
    start_time,
    end_time,
    tags
  )
VALUES (
    '2c20e6d2-7bce-47b7-b02d-7f45fb106df5',
    'Travel Vlog',
    'Join me on my adventures around the world',
    999,
    1,
    'https://video.com/travel',
    NOW(),
    'English',
    999,
    '00:00:00',
    '00:05:00',
    'travel;adventure'
  );