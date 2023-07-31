-- Create users
INSERT INTO
  users (id, name, email, password)
VALUES
  (1, 'User1', 'user1@test.com', 'Password1'),
  (2, 'User2', 'user2@test.com', 'Password2'),
  (3, 'User3', 'user3@test.com', 'Password3');

-- Create channels
INSERT INTO
  channels (id, name, creator_id, refresh_token)
VALUES
  (1, 'Channel1', 1, 'abD5Fjkl2sjhfKpR'),
  (2, 'Channel2', 2, 'pWm9RtqA3jkl9VxS'),
  (3, 'Channel3', 3, 'l0iFj3RtD5Ahkfms'),
  (4, 'Channel4', 1, 'kGmj5Fw4skHty7mN'),
  (5, 'Channel5', 2, 'Hjf9smIwN2RtY4pB'),
  (6, 'Channel6', 3, 'SjyTrg5pBzWf8KlA'),
  (7, 'Channel7', 1, 'fwTd8pL5mB1Ry6sJ'),
  (8, 'Channel8', 2, 'M8jGhW5fPrB3kNya'),
  (9, 'Channel9', 3, 'zXQn2eTrg8yBmK7L'),
  (10, 'Channel10', 1, 'A5PvN8SfK6Tr3xjW'),
  (11, 'Channel11', 2, 'W7Rsxj5hB3pLkTqA'),
  (12, 'Channel12', 3, 'G1Rr5pAqJ5hTfBzV'),
  (13, 'Channel13', 1, 'Fb2zHjRmW8rQxG3K'),
  (14, 'Channel14', 2, 'LsU5mB7XjKpVrG2z'),
  (15, 'Channel15', 3, 'HmW5aX3PjRvNf8Kl'),
  (16, 'Channel16', 1, 'K9rYbA6xL1sTjH2G'),
  (17, 'Channel17', 2, 'V8sJzFn2rG5bXpAq'),
  (18, 'Channel18', 3, 'L7KjW5fRm2HxPqA8'),
  (19, 'Channel19', 1, 'M5hGjZ3TrX7kPqA2'),
  (20, 'Channel20', 2, 'RjG5KlW4yBxN2HsL'),
  (21, 'Channel21', 3, 'V5HnFb8xSjZrKqG2'),
  (22, 'Channel22', 1, 'M3XjTqRfL7bWgH9r'),
  (23, 'Channel23', 2, 'L9mBhG2qJ7NwR3yA'),
  (24, 'Channel24', 3, 'PjR5BzWqT9KfHm7L'),
  (25, 'Channel25', 1, 'SfKp3QxL5A7jRgTb'),
  (26, 'Channel26', 2, 'JrTbSjWgZ8L5mPqA'),
  (27, 'Channel27', 3, 'RgTbL7yNzXkPqA6j'),
  (28, 'Channel28', 1, 'M9rHfQb5XjSgZ2Lp'),
  (29, 'Channel29', 2, 'Z3KqHjRfG2sXmB5p'),
  (30, 'Channel30', 3, 'L5XjTqHmB7KpR2gF'),
  (31, 'Channel25', 1, 'J6rTbG2zH8LqQxKp'),
  (32, 'Channel26', 2, 'Z7XjRfTbP6yHmL5q'),
  (33, 'Channel27', 3, 'H4kPqA8rTjX7LmBz'),
  (34, 'Channel28', 1, 'G5zXjPqR9HbTmK8s'),
  (35, 'Channel29', 2, 'N7bZ4rXjG2qLmH5p'),
  (36, 'Channel30', 3, 'K3bHjZ7rTgW8XpNq');

-- Create channels with deleted_at
INSERT INTO
  channels (id, name, creator_id, deleted_at)
VALUES
  (37, 'Channel1', 1, now()),
  (38, 'Channel2', 2, now()),
  (39, 'Channel3', 3, now()),
  (40, 'Channel4', 1, now()),
  (41, 'Channel5', 2, now()),
  (42, 'Channel6', 3, now()),
  (43, 'Channel7', 1, now()),
  (44, 'Channel8', 2, now()),
  (45, 'Channel9', 3, now());

--create channels with error
INSERT INTO
  channels (id, name, creator_id, refresh_token, error)
VALUES
  (46, 'Channel46', 1, 'K4bHjZ7rTgW8XpNq', true),
  (47, 'Channel47', 2, 'K4bHjZ7rTgW8XpNq', true),
  (48, 'Channel48', 3, 'K4bHjZ7rTgW8XpNq', true),
  (49, 'Channel49', 1, 'K4bHjZ7rTgW8XpNq', true),
  (50, 'Channel50', 2, 'K4bHjZ7rTgW8XpNq', true),
  (51, 'Channel51', 3, 'K4bHjZ7rTgW8XpNq', true);

--create channels without refresh_token
INSERT INTO
  channels (id, name, creator_id, csrf_token)
VALUES
  (52, 'Channel52', 1, 'K4bHjZ7rTgW8XpNq'),
  (53, 'Channel53', 2, 'K4bHjZ7rTgW8XpNq'),
  (54, 'Channel54', 3, 'K4bHjZ7rTgW8XpNq'),
  (55, 'Channel55', 1, 'K4bHjZ7rTgW8XpNq'),
  (56, 'Channel56', 2, 'K4bHjZ7rTgW8XpNq'),
  (57, 'Channel57', 3, 'K4bHjZ7rTgW8XpNq');