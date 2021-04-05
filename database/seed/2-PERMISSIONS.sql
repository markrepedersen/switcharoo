CREATE TABLE IF NOT EXISTS Permissions (
       id serial PRIMARY KEY,
       name varchar(25) UNIQUE NOT NULL
);

CREATE TABLE IF NOT EXISTS UserPermissions (
       user_id uuid NOT NULL REFERENCES Users(id) ON DELETE CASCADE,
       permission_id INTEGER NOT NULL REFERENCES Permissions(id) ON DELETE CASCADE,
       PRIMARY KEY (user_id, permission_id)
);

-- Permissions
INSERT INTO Permissions(name) VALUES ('Read');
INSERT INTO Permissions(name) VALUES ('Add');
INSERT INTO Permissions(name) VALUES ('Remove');
INSERT INTO Permissions(name) VALUES ('Update');

-- Roles (Permissions that are prefixed by ROLE_)
INSERT INTO Permissions(name) VALUES ('ROLE_Guest');
INSERT INTO Permissions(name) VALUES ('ROLE_Admin');
INSERT INTO Permissions(name) VALUES ('ROLE_Developer');
