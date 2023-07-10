-- Create users
INSERT INTO users (id, name, email, password)
VALUES
  (1, 'User1', 'user1@test.com', 'Password1'),
  (2, 'User2', 'user2@test.com', 'Password2'),
  (3, 'User3', 'user3@test.com', 'Password3');

-- Create channels
INSERT INTO channels (id, name, creator_id)
VALUES
  (1, 'Channel1', 1),
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
  (20, 'Channel20', 2);