INSERT INTO users (
    id,
    name,
    email,
    password,
    role,
    forgot_token,
    forgot_token_expires_at
  )
VALUES (
    6666,
    'TestUser',
    'test@test.com',
    '$2b$12$.jvb858VF4tanKNd11Vp4eDYyhg.KuFgOG8AhgJCvj/cJV47Sqtby',
    'USER',
    'd1596e0d4280f2bd2d311ce0819f23bde0dc834d8254b92924088de94c38d922',
    NOW() + INTERVAL '1 DAY'
  ),
  (
    7777,
    'TestUser2',
    'test2@test.com',
    '$2b$12$.jvb858VF4tanKNd11Vp4eDYyhg.KuFgOG8AhgJCvj/cJV47Sqtby',
    'USER',
    '72a0609962dde107e39653651588536e9ea0269e8214e3d6b547a8dcbe652a49',
    NOW()
  ),
  (
    8888,
    'TestUser3',
    'test3@test.com',
    '$2b$12$.jvb858VF4tanKNd11Vp4eDYyhg.KuFgOG8AhgJCvj/cJV47Sqtby',
    'USER',
    'dfdd6acbdefd97e02076a54a6d2fefa86de49e922181d1a2af69b0fe92f8e199',
    NOW() - INTERVAL '1 DAY'
  ),
  (
    9999,
    'TestUser4',
    'test4@test.com',
    '$2b$12$.jvb858VF4tanKNd11Vp4eDYyhg.KuFgOG8AhgJCvj/cJV47Sqtby',
    'USER',
    '859a0861421399f3ff5263f15ff9b4fde88679080d65bfee08429d0077a32147',
    NOW() + INTERVAL '1 MINUTE'
  );