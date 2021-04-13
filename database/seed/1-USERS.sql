CREATE TABLE IF NOT EXISTS Users (
       id uuid PRIMARY KEY,
       email varchar(100) UNIQUE NOT NULL,
       password varchar(120) NOT NULL,
       tenant_id uuid NOT NULL
);
