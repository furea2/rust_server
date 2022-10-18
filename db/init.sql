--ユーザーの作成
CREATE USER glaceon;
--DBの作成
CREATE DATABASE glaceon_db;
--ユーザーにDBの権限をまとめて付与
GRANT ALL PRIVILEGES ON DATABASE glaceon_db TO glaceon;
--ユーザーを切り替え
\c glaceon
--テーブルを作成
CREATE TABLE book (
  id integer, 
  name varchar(30)
);
--テーブルにデータを挿入
INSERT INTO book VALUES (1,'The Very Hungry Caterpillar');
