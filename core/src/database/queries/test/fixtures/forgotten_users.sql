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
    'd1596e0d4280f2bd2d311ce0819f23bde0dc834d8254b92924088de94c38d923',
    NOW()
  ),
  (
    8888,
    'TestUser3',
    'test3@test.com',
    '$2b$12$.jvb858VF4tanKNd11Vp4eDYyhg.KuFgOG8AhgJCvj/cJV47Sqtby',
    'USER',
    'd1596e0d4280f2bd2d311ce0819f23bde0dc834d8254b92924088de94c38d924',
    NOW() - INTERVAL '1 DAY'
  ),
  (
    9999,
    'TestUser4',
    'test4@test.com',
    '$2b$12$.jvb858VF4tanKNd11Vp4eDYyhg.KuFgOG8AhgJCvj/cJV47Sqtby',
    'USER',
    'd1596e0d4280f2bd2d311ce0819f23bde0dc834d8254b92924088de94c38d925',
    NOW() + INTERVAL '1 MINUTE'
  );