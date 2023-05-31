-- Your SQL goes here
-- generate 10 random users
INSERT INTO users (u_nickname, u_email, u_password, u_avatar, u_creattime, u_lastlogin)
SELECT
  'test' || random()::text,
  md5(random()::text),
  md5(random()::text),
  md5(random()::text),
  now(),
  now()
FROM generate_series(1, 10);
