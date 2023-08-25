-- Create users
INSERT INTO
  users (id, name, email, password)
VALUES
  (1, 'User1', 'user1@test.com', 'Password1'),
  (2, 'User2', 'user2@test.com', 'Password2'),
  (3, 'User3', 'user3@test.com', 'Password3');

-- Create channels
INSERT INTO
  channels (id, name, creator_id, auth)
VALUES
  (
    1,
    'Channel1',
    1,
    jsonb_build_object(
      'type',
      'OAUTH2',
      'data',
      jsonb_build_object(
        'refresh_token',
        'abD5Fjkl2sjhfKpR'
      )
    )
  ),
  (
    2,
    'Channel2',
    2,
    jsonb_build_object(
      'type',
      'OAUTH2',
      'data',
      jsonb_build_object('refresh_token', 'pWm9RtqA3jkl9VxS')
    )
  ),
  (
    3,
    'Channel3',
    3,
    jsonb_build_object(
      'type',
      'OAUTH2',
      'data',
      jsonb_build_object('refresh_token', 'pWm9RtqA3jkl9VxS')
    )
  ),
  (
    4,
    'Channel4',
    1,
    jsonb_build_object(
      'type',
      'OAUTH2',
      'data',
      jsonb_build_object('refresh_token', 'pWm9RtqA3jkl9VxS')
    )
  ),
  (
    5,
    'Channel5',
    2,
    jsonb_build_object(
      'type',
      'OAUTH2',
      'data',
      jsonb_build_object('refresh_token', 'pWm9RtqA3jkl9VxS')
    )
  ),
  (
    6,
    'Channel6',
    3,
    jsonb_build_object(
      'type',
      'OAUTH2',
      'data',
      jsonb_build_object('refresh_token', 'pWm9RtqA3jkl9VxS')
    )
  ),
  (
    7,
    'Channel7',
    1,
    jsonb_build_object(
      'type',
      'OAUTH2',
      'data',
      jsonb_build_object('refresh_token', 'pWm9RtqA3jkl9VxS')
    )
  ),
  (
    8,
    'Channel8',
    2,
    jsonb_build_object(
      'type',
      'OAUTH2',
      'data',
      jsonb_build_object('refresh_token', 'pWm9RtqA3jkl9VxS')
    )
  ),
  (
    9,
    'Channel9',
    3,
    jsonb_build_object(
      'type',
      'OAUTH2',
      'data',
      jsonb_build_object('refresh_token', 'pWm9RtqA3jkl9VxS')
    )
  ),
  (
    10,
    'Channel10',
    1,
    jsonb_build_object(
      'type',
      'OAUTH2',
      'data',
      jsonb_build_object('refresh_token', 'pWm9RtqA3jkl9VxS')
    )
  ),
  (
    11,
    'Channel11',
    2,
    jsonb_build_object(
      'type',
      'OAUTH2',
      'data',
      jsonb_build_object('refresh_token', 'pWm9RtqA3jkl9VxS')
    )
  ),
  (
    12,
    'Channel12',
    3,
    jsonb_build_object(
      'type',
      'OAUTH2',
      'data',
      jsonb_build_object('refresh_token', 'pWm9RtqA3jkl9VxS')
    )
  ),
  (
    13,
    'Channel13',
    1,
    jsonb_build_object(
      'type',
      'OAUTH2',
      'data',
      jsonb_build_object('refresh_token', 'pWm9RtqA3jkl9VxS')
    )
  ),
  (
    14,
    'Channel14',
    2,
    jsonb_build_object(
      'type',
      'OAUTH2',
      'data',
      jsonb_build_object('refresh_token', 'pWm9RtqA3jkl9VxS')
    )
  ),
  (
    15,
    'Channel15',
    3,
    jsonb_build_object(
      'type',
      'OAUTH2',
      'data',
      jsonb_build_object('refresh_token', 'pWm9RtqA3jkl9VxS')
    )
  ),
  (
    16,
    'Channel16',
    1,
    jsonb_build_object(
      'type',
      'OAUTH2',
      'data',
      jsonb_build_object('refresh_token', 'pWm9RtqA3jkl9VxS')
    )
  ),
  (
    17,
    'Channel17',
    2,
    jsonb_build_object(
      'type',
      'OAUTH2',
      'data',
      jsonb_build_object('refresh_token', 'pWm9RtqA3jkl9VxS')
    )
  ),
  (
    18,
    'Channel18',
    3,
    jsonb_build_object(
      'type',
      'OAUTH2',
      'data',
      jsonb_build_object('refresh_token', 'pWm9RtqA3jkl9VxS')
    )
  ),
  (
    19,
    'Channel19',
    1,
    jsonb_build_object(
      'type',
      'OAUTH2',
      'data',
      jsonb_build_object('refresh_token', 'pWm9RtqA3jkl9VxS')
    )
  ),
  (
    20,
    'Channel20',
    2,
    jsonb_build_object(
      'type',
      'OAUTH2',
      'data',
      jsonb_build_object('refresh_token', 'pWm9RtqA3jkl9VxS')
    )
  ),
  (
    21,
    'Channel21',
    3,
    jsonb_build_object(
      'type',
      'OAUTH2',
      'data',
      jsonb_build_object('refresh_token', 'pWm9RtqA3jkl9VxS')
    )
  ),
  (
    22,
    'Channel22',
    1,
    jsonb_build_object(
      'type',
      'OAUTH2',
      'data',
      jsonb_build_object('refresh_token', 'pWm9RtqA3jkl9VxS')
    )
  ),
  (
    23,
    'Channel23',
    2,
    jsonb_build_object(
      'type',
      'OAUTH2',
      'data',
      jsonb_build_object('refresh_token', 'pWm9RtqA3jkl9VxS')
    )
  ),
  (
    24,
    'Channel24',
    3,
    jsonb_build_object(
      'type',
      'OAUTH2',
      'data',
      jsonb_build_object('refresh_token', 'pWm9RtqA3jkl9VxS')
    )
  ),
  (
    25,
    'Channel25',
    1,
    jsonb_build_object(
      'type',
      'OAUTH2',
      'data',
      jsonb_build_object('refresh_token', 'pWm9RtqA3jkl9VxS')
    )
  ),
  (
    26,
    'Channel26',
    2,
    jsonb_build_object(
      'type',
      'OAUTH2',
      'data',
      jsonb_build_object('refresh_token', 'pWm9RtqA3jkl9VxS')
    )
  ),
  (
    27,
    'Channel27',
    3,
    jsonb_build_object(
      'type',
      'OAUTH2',
      'data',
      jsonb_build_object('refresh_token', 'pWm9RtqA3jkl9VxS')
    )
  ),
  (
    28,
    'Channel28',
    1,
    jsonb_build_object(
      'type',
      'OAUTH2',
      'data',
      jsonb_build_object('refresh_token', 'pWm9RtqA3jkl9VxS')
    )
  ),
  (
    29,
    'Channel29',
    2,
    jsonb_build_object(
      'type',
      'OAUTH2',
      'data',
      jsonb_build_object('refresh_token', 'pWm9RtqA3jkl9VxS')
    )
  ),
  (
    30,
    'Channel30',
    3,
    jsonb_build_object(
      'type',
      'OAUTH2',
      'data',
      jsonb_build_object('refresh_token', 'pWm9RtqA3jkl9VxS')
    )
  ),
  (
    31,
    'Channel25',
    1,
    jsonb_build_object(
      'type',
      'OAUTH2',
      'data',
      jsonb_build_object('refresh_token', 'pWm9RtqA3jkl9VxS')
    )
  ),
  (
    32,
    'Channel26',
    2,
    jsonb_build_object(
      'type',
      'OAUTH2',
      'data',
      jsonb_build_object('refresh_token', 'pWm9RtqA3jkl9VxS')
    )
  ),
  (
    33,
    'Channel27',
    3,
    jsonb_build_object(
      'type',
      'OAUTH2',
      'data',
      jsonb_build_object('refresh_token', 'pWm9RtqA3jkl9VxS')
    )
  ),
  (
    34,
    'Channel28',
    1,
    jsonb_build_object(
      'type',
      'OAUTH2',
      'data',
      jsonb_build_object('refresh_token', 'pWm9RtqA3jkl9VxS')
    )
  ),
  (
    35,
    'Channel29',
    2,
    jsonb_build_object(
      'type',
      'OAUTH2',
      'data',
      jsonb_build_object('refresh_token', 'pWm9RtqA3jkl9VxS')
    )
  ),
  (
    36,
    'Channel30',
    3,
    jsonb_build_object(
      'type',
      'OAUTH2',
      'data',
      jsonb_build_object('refresh_token', 'pWm9RtqA3jkl9VxS')
    )
  );

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
  channels (id, name, creator_id, auth, error)
VALUES
  (
    46,
    'Channel46',
    1,
    jsonb_build_object(
      'type',
      'OAUTH2',
      'data',
      jsonb_build_object('refresh_token', 'pWm9RtqA3jkl9VxS')
    ),
    true
  ),
  (
    47,
    'Channel47',
    2,
    jsonb_build_object(
      'type',
      'OAUTH2',
      'data',
      jsonb_build_object('refresh_token', 'pWm9RtqA3jkl9VxS')
    ),
    true
  ),
  (
    48,
    'Channel48',
    3,
    jsonb_build_object(
      'type',
      'OAUTH2',
      'data',
      jsonb_build_object('refresh_token', 'pWm9RtqA3jkl9VxS')
    ),
    true
  ),
  (
    49,
    'Channel49',
    1,
    jsonb_build_object(
      'type',
      'OAUTH2',
      'data',
      jsonb_build_object('refresh_token', 'pWm9RtqA3jkl9VxS')
    ),
    true
  ),
  (
    50,
    'Channel50',
    2,
    jsonb_build_object(
      'type',
      'OAUTH2',
      'data',
      jsonb_build_object('refresh_token', 'pWm9RtqA3jkl9VxS')
    ),
    true
  ),
  (
    51,
    'Channel51',
    3,
    jsonb_build_object(
      'type',
      'OAUTH2',
      'data',
      jsonb_build_object('refresh_token', 'pWm9RtqA3jkl9VxS')
    ),
    true
  );

--create channels without refresh_token
INSERT INTO
  channels (id, name, creator_id, auth)
VALUES
  (
    52,
    'Channel52',
    1,
    jsonb_build_object(
      'type',
      'OAUTH2',
      'data',
      jsonb_build_object('csrf_token', 'K4bHjZ7rTgW8XpNq')
    )
  ),
  (
    53,
    'Channel53',
    2,
    jsonb_build_object(
      'type',
      'OAUTH2',
      'data',
      jsonb_build_object('csrf_token', 'K4bHjZ7rTgW8XpNq')
    )
  ),
  (
    54,
    'Channel54',
    3,
    jsonb_build_object(
      'type',
      'OAUTH2',
      'data',
      jsonb_build_object('csrf_token', 'K4bHjZ7rTgW8XpNq')
    )
  ),
  (
    55,
    'Channel55',
    1,
    jsonb_build_object(
      'type',
      'OAUTH2',
      'data',
      jsonb_build_object('csrf_token', 'K4bHjZ7rTgW8XpNq')
    )
  ),
  (
    56,
    'Channel56',
    2,
    jsonb_build_object(
      'type',
      'OAUTH2',
      'data',
      jsonb_build_object('csrf_token', 'K4bHjZ7rTgW8XpNq')
    )
  ),
  (
    57,
    'Channel57',
    3,
    jsonb_build_object(
      'type',
      'OAUTH2',
      'data',
      jsonb_build_object('csrf_token', 'K4bHjZ7rTgW8XpNq')
    )
  );