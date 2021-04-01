CREATE TABLE Users IF NOT EXISTS (id serial PRIMARY KEY, email varchar(100) NOT NULL, hashpass varchar(100) NOT NULL);

INSERT INTO Users(email, hashpass) VALUES ('admin', 'admin');
