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
  channels (id, name, creator_id)
VALUES
  (666, 'TestChannel', 666);

-- with url 
INSERT INTO
  VIDEOS (
    id,
    title,
    url,
    description,
    user_id,
    channel_id,
    original_url,
    original_duration,
    start_time,
    end_time,
    tags
  )
VALUES
  (
    '806b5a48-f221-11ed-a05b-0242ac120096',
    'Space Tourism Test',
    'https://www.youtube.com/watch?v=1234567890',
    'This is a test video about space tourism',
    666,
    666,
    'https://www.youtube.com/watch?v=1234567890',
    '00:10:00',
    '00:00:00',
    '00:10:00',
    'spacetourism'
  );

-- without url
INSERT INTO
  VIDEOS (
    id,
    title,
    description,
    user_id,
    channel_id,
    original_url,
    original_duration,
    start_time,
    end_time,
    tags
  )
VALUES
  (
    '806b5a6c-f221-11ed-a05b-0242ac120095',
    'Hyperloop Competition Test',
    'This is a test video of a Hyperloop competition',
    666,
    666,
    'https://www.youtube.com/watch?v=1234567890',
    '00:10:00',
    '00:00:00',
    '00:10:00',
    'hyperloop;competition'
  ),
  (
    '806b5a8e-f221-11ed-a05b-0242ac120094',
    'Mars Rover Exploration Test',
    'This is a test video of a Mars rover exploration',
    666,
    666,
    'https://www.youtube.com/watch?v=1234567890',
    '00:10:00',
    '00:00:00',
    '00:10:00',
    'mars;rover;exploration'
  ),
  (
    '806b5ab0-f221-11ed-a05b-0242ac120093',
    'Neuralink Implant Test',
    'This is a test video about a Neuralink implant',
    666,
    666,
    'https://www.youtube.com/watch?v=1234567890',
    '00:10:00',
    '00:00:00',
    '00:10:00',
    'neuralink;implant'
  ),
  (
    '806b5ad4-f221-11ed-a05b-0242ac120092',
    'Autonomous Car Test',
    'This is a test video of an autonomous car',
    666,
    666,
    'https://www.youtube.com/watch?v=1234567890',
    '00:10:00',
    '00:00:00',
    '00:10:00',
    'autonomous;car'
  ),
  (
    '806b5af6-f221-11ed-a05b-0242ac120091',
    'SpaceX Starlink Test',
    'This is a test video about SpaceX Starlink',
    666,
    666,
    'https://www.youtube.com/watch?v=1234567890',
    '00:10:00',
    '00:00:00',
    '00:10:00',
    'spacex;starlink'
  ),
  (
    '806b5b1c-f221-11ed-a05b-0242ac120090',
    'AI Artwork Test',
    'This is a test video of AI-generated artwork',
    666,
    666,
    'https://www.youtube.com/watch?v=1234567890',
    '00:10:00',
    '00:00:00',
    '00:10:00',
    'ai;artwork'
  ),
  (
    '806b5b3c-f221-11ed-a05b-0242ac120089',
    'Renewable Energy Test',
    'This is a test video about renewable energy',
    666,
    666,
    'https://www.youtube.com/watch?v=1234567890',
    '00:10:00',
    '00:00:00',
    '00:10:00',
    'renewable;energy'
  ),
  (
    '806b5b5c-f221-11ed-a05b-0242ac120088',
    'Journey to the Center of the Earth Test',
    'This is a test video about a journey to the center of the Earth',
    666,
    666,
    'https://www.youtube.com/watch?v=1234567890',
    '00:10:00',
    '00:00:00',
    '00:10:00',
    'journey;center;earth'
  ),
  (
    '806b5b7c-f221-11ed-a05b-0242ac120087',
    'Cybersecurity Test',
    'This is a test video about cybersecurity',
    666,
    666,
    'https://www.youtube.com/watch?v=1234567890',
    '00:10:00',
    '00:00:00',
    '00:10:00',
    'cybersecurity'
  ),
  (
    '806b5b9c-f221-11ed-a05b-0242ac120086',
    'Artificial Intelligence Ethics Test',
    'This is a test video about artificial intelligence ethics',
    666,
    666,
    'https://www.youtube.com/watch?v=1234567890',
    '00:10:00',
    '00:00:00',
    '00:10:00',
    'artificial;intelligence;ethics'
  ),
  (
    '806b5bbc-f221-11ed-a05b-0242ac120085',
    'Robotic Surgery Test',
    'This is a test video of robotic surgery',
    666,
    666,
    'https://www.youtube.com/watch?v=1234567890',
    '00:10:00',
    '00:00:00',
    '00:10:00',
    'robotic;surgery'
  ),
  (
    '806b5bdc-f221-11ed-a05b-0242ac120084',
    'Ocean Exploration Test',
    'This is a test video about ocean exploration',
    666,
    666,
    'https://www.youtube.com/watch?v=1234567890',
    '00:10:00',
    '00:00:00',
    '00:10:00',
    'ocean;exploration'
  ),
  (
    '806b5bfc-f221-11ed-a05b-0242ac120083',
    'Virtual Reality Gaming Test',
    'This is a test video of virtual reality gaming',
    666,
    666,
    'https://www.youtube.com/watch?v=1234567890',
    '00:10:00',
    '00:00:00',
    '00:10:00',
    'virtual;reality;gaming'
  ),
  (
    '806b5c1c-f221-11ed-a05b-0242ac120082',
    'Climate Change Solutions Test',
    'This is a test video about climate change solutions',
    666,
    666,
    'https://www.youtube.com/watch?v=1234567890',
    '00:10:00',
    '00:00:00',
    '00:10:00',
    'climate;change;solutions'
  ),
  (
    '806b5c3c-f221-11ed-a05b-0242ac120081',
    'Space Colonization Test',
    'This is a test video about space colonization',
    666,
    666,
    'https://www.youtube.com/watch?v=1234567890',
    '00:10:00',
    '00:00:00',
    '00:10:00',
    'space;colonization'
  ),
  (
    '806b5c5c-f221-11ed-a05b-0242ac120080',
    'Renewable Energy Innovations Test',
    'This is a test video of renewable energy innovations',
    666,
    666,
    'https://www.youtube.com/watch?v=1234567890',
    '00:10:00',
    '00:00:00',
    '00:10:00',
    'renewable;energy;innovations'
  ),
  (
    '806b5c7c-f221-11ed-a05b-0242ac120079',
    'Smart Homes Test',
    'This is a test video about smart homes',
    666,
    666,
    'https://www.youtube.com/watch?v=1234567890',
    '00:10:00',
    '00:00:00',
    '00:10:00',
    'smart;homes'
  ),
  (
    '806b5c9c-f221-11ed-a05b-0242ac120078',
    'Medical Breakthroughs Test',
    'This is a test video about medical breakthroughs',
    666,
    666,
    'https://www.youtube.com/watch?v=1234567890',
    '00:10:00',
    '00:00:00',
    '00:10:00',
    'medical;breakthroughs'
  ),
  (
    '806b5cbc-f221-11ed-a05b-0242ac120077',
    'Space Telescopes Test',
    'This is a test video about space telescopes',
    666,
    666,
    'https://www.youtube.com/watch?v=1234567890',
    '00:10:00',
    '00:00:00',
    '00:10:00',
    'space;telescopes'
  );

INSERT INTO
  VIDEOS (
    id,
    title,
    description,
    user_id,
    channel_id,
    deleted_at,
    original_url,
    original_duration,
    start_time,
    end_time,
    tags
  )
VALUES
  (
    '806b5cdc-f221-11ed-a05b-0242ac120076',
    'Space Tourism Test',
    'This is a test video about space tourism',
    666,
    666,
    '2021-09-01 00:00:00',
    'https://www.youtube.com/watch?v=1234567890',
    '00:10:00',
    '00:00:00',
    '00:10:00',
    'spacetourism'
  );

INSERT INTO
  VIDEOS (
    id,
    title,
    description,
    user_id,
    channel_id,
    deleted_at,
    original_url,
    original_duration,
    start_time,
    end_time,
    tags
  )
VALUES
  (
    '806b5cfc-f221-11ed-a05b-0242ac120075',
    'Hyperloop Competition Test',
    'This is a test video of a Hyperloop competition',
    666,
    666,
    NULL,
    'https://www.youtube.com/watch?v=1234567890',
    '00:10:00',
    '00:00:00',
    '00:10:00',
    'hyperloop;competition'
  );