-- Create users
INSERT INTO users (id, name, email, password)
VALUES (1, 'User1', 'user1@test.com', 'Password1'),
  (2, 'User2', 'user2@test.com', 'Password2'),
  (3, 'User3', 'user3@test.com', 'Password3');

-- Create channels
INSERT INTO channels (id, name, creator_id)
VALUES (1, 'Channel1', 1),
  (2, 'Channel2', 2),
  (3, 'Channel3', 3),
  (4, 'Channel4', 1),
  (5, 'Channel5', 2),
  (6, 'Channel6', 3),
  (7, 'Channel7', 1),
  (8, 'Channel8', 2),
  (9, 'Channel9', 3),
  (10, 'Channel10', 1),
  (11, 'Channel11', 2),
  (12, 'Channel12', 3),
  (13, 'Channel13', 1),
  (14, 'Channel14', 2),
  (15, 'Channel15', 3),
  (16, 'Channel16', 1),
  (17, 'Channel17', 2),
  (18, 'Channel18', 3),
  (19, 'Channel19', 1),
  (20, 'Channel20', 2),
  (21, 'Channel21', 3),
  (22, 'Channel22', 1),
  (23, 'Channel23', 2),
  (24, 'Channel24', 3),
  (25, 'Channel25', 1),
  (26, 'Channel26', 2),
  (27, 'Channel27', 3),
  (28, 'Channel28', 1),
  (29, 'Channel29', 2),
  (30, 'Channel30', 3),
  (31, 'Channel25', 1),
  (32, 'Channel26', 2),
  (33, 'Channel27', 3),
  (34, 'Channel28', 1),
  (35, 'Channel29', 2),
  (36, 'Channel30', 3);

-- Create channels with deleted_at
INSERT INTO channels (id, name, creator_id, deleted_at)
VALUES (37, 'Channel1', 1, now()),
  (38, 'Channel2', 2, now()),
  (39, 'Channel3', 3, now()),
  (40, 'Channel4', 1, now()),
  (41, 'Channel5', 2, now()),
  (42, 'Channel6', 3, now()),
  (43, 'Channel7', 1, now()),
  (44, 'Channel8', 2, now()),
  (45, 'Channel9', 3, now());