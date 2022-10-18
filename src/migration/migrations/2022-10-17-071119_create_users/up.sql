-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  username VARCHAR NOT NULL
);
INSERT INTO users VALUES (0,'admin');
INSERT INTO users VALUES (1,'furea2');
INSERT INTO users VALUES (2,'glaceon');
