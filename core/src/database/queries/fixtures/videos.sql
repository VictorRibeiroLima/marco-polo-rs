#[sqlx::test(migrations = "../migrations", fixtures("videos"))]
INSERT INTO VIDEOS (id, title, description) 
VALUES ('806b57d2-f221-11ed-a05b-0242ac120003', 'Elon Musk Test', 'This is a test video about Elon Musk');