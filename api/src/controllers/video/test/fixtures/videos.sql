INSERT INTO USERS (id, name, email, password) 
VALUES (456, 'TestUser', 'test@gmail.com', '$2b$12$.jvb858VF4tanKNd11Vp4eDYyhg.KuFgOG8AhgJCvj/cJV47Sqtby'); --99020711Aa@

INSERT INTO USERS (id, name, email, password)
VALUES (789, 'AdminUser', 'admin@example.com', '$2b$12$JrqfLc8Mm0UhrKJqjIuYHuTlWlTNVW9bPb3W1PZJfV0XNQcHCrLI6'); --99020711Aa@

INSERT INTO CHANNELS (id,name,creator_id) VALUES (666,'TestChannel',456);

INSERT INTO CHANNELS (id,name,creator_id) VALUES (678,'TestChannel',789);

INSERT INTO VIDEOS (id, title, description, user_id, channel_id, url, language, created_at, updated_at, uploaded_at) 
VALUES ('806b57d2-f221-11ed-a05b-0242ac120003','Elon Musk Test', 'This is a test video about Elon Musk', 456, 666, 'https://video.com','English', '2022-01-01', '2022-01-01','2022-01-01');

INSERT INTO VIDEOS (id, title, description, user_id, channel_id, url, language, created_at, updated_at, uploaded_at)
VALUES ('b7a720e3-010e-4d88-919b-7aee4d7a3144', 'Cats Compilation', 'A compilation of funny cat videos', 456, 666, 'https://video.com/cats', 'English', '2022-01-02', '2022-01-02', '2022-01-02');

INSERT INTO VIDEOS (id, title, description, user_id, channel_id, url, language, created_at, updated_at, uploaded_at)
VALUES ('07cc7053-6aee-4e27-9310-0e8593aee422', 'Cooking Tutorial', 'Learn how to cook a delicious meal', 456, 666, 'https://video.com/cooking', 'English', '2022-01-03', '2022-01-03', '2022-01-03');

INSERT INTO VIDEOS (id, title, description, user_id, channel_id, url, language, created_at, updated_at, uploaded_at)
VALUES ('9b594b49-c2b9-40a1-a20d-8d18a50dcd8d', 'Gardening Tips', 'Discover helpful tips for gardening enthusiasts', 456, 666, 'https://video.com/gardening', 'English', '2022-01-04', '2022-01-04', '2022-01-04');

INSERT INTO VIDEOS (id, title, description, user_id, channel_id, url, language, created_at, updated_at, uploaded_at)
VALUES ('e4a399d1-7d97-432d-8681-30a6a94f88f5', 'Fitness Workout', 'Follow along with this intense workout routine', 456, 666, 'https://video.com/fitness', 'English', '2022-01-05', '2022-01-05', '2022-01-05');

INSERT INTO VIDEOS (id, title, description, user_id, channel_id, url, language, created_at, updated_at, uploaded_at)
VALUES ('2c20e6d2-7bce-47b7-b02d-7f45fb106df7', 'Travel Vlog', 'Join me on my adventures around the world', 456, 666, 'https://video.com/travel', 'English', '2022-01-06', '2022-01-06', '2022-01-06');

INSERT INTO VIDEOS (id, title, description, user_id, channel_id, url, language, created_at, updated_at, uploaded_at)
VALUES ('fe0a2ab6-3d94-4db7-8a89-58b77a0f367e', 'Science Experiments', 'Witness amazing scientific experiments in action', 456, 666, 'https://video.com/science', 'English', '2022-01-07', '2022-01-07', '2022-01-07');

INSERT INTO VIDEOS (id, title, description, user_id, channel_id, url, language, created_at, updated_at, uploaded_at)
VALUES ('ac9a10b9-17e9-412f-a166-144a07a30e6d', 'Funny Pranks', 'Get ready to laugh with these hilarious pranks', 456, 666, 'https://video.com/pranks', 'English', '2022-01-08', '2022-01-08', '2022-01-08');

INSERT INTO VIDEOS (id, title, description, user_id, channel_id, url, language, created_at, updated_at, uploaded_at)
VALUES ('05f06d54-0c32-485b-bde1-22bb8da09a5c', 'Music Performance', 'Enjoy a captivating live music performance', 456, 666, 'https://video.com/music', 'English', '2022-01-09', '2022-01-09', '2022-01-09');

INSERT INTO VIDEOS (id, title, description, user_id, channel_id, url, language, created_at, updated_at, uploaded_at)
VALUES ('1c7b8db2-bd92-434b-9b4f-63d643a6f81d', 'Art Tutorial', 'Learn how to create stunning artworks', 456, 666, 'https://video.com/art', 'English', '2022-01-10', '2022-01-10', '2022-01-10');

-- Inserting videos into the VIDEOS table
INSERT INTO VIDEOS (id, title, description, user_id, channel_id, url, language, created_at, updated_at, uploaded_at)
VALUES ('4e87a122-6f59-4a48-9ff6-6a5c9d7082e0', 'Gaming Highlights', 'Watch exciting highlights from the world of gaming', 789, 678, 'https://video.com/gaming', 'English', '2022-01-11', '2022-01-11', '2022-01-11');

INSERT INTO VIDEOS (id, title, description, user_id, channel_id, url, language, created_at, updated_at, uploaded_at)
VALUES ('09a9e5f5-2c3b-4a54-bb1f-8a4d67c6156f', 'Nature Documentary', 'Explore the wonders of nature in this documentary', 789, 678, 'https://video.com/nature', 'English', '2022-01-12', '2022-01-12', '2022-01-12');

INSERT INTO VIDEOS (id, title, description, user_id, channel_id, url, language, created_at, updated_at, uploaded_at)
VALUES ('48f6cbe7-4b88-45f1-8b7e-cac11dbf8f2e', 'Tech Reviews', 'Get the latest insights into the world of technology', 789, 678, 'https://video.com/tech', 'English', '2022-01-13', '2022-01-13', '2022-01-13');
