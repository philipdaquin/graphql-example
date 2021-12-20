CREATE TABLE teams (
    id SERIAL PRIMARY KEY
    name VARCHAR NOT NULL
);
CREATE TABLE members (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    knockouts INT NOT NULL DEFAULT 0,
    team_id INT NOT NULL,
    FOREIGN KEY (team_id) REFERENCES teams(id)
);


INSERT INTO teams(id, name) VALUES (1, 'HEROES');
INSERT INTO members(name, knockouts, team_id) VALUES ('Link', 14, 1);
INSERT INTO members(name, knockouts, team_id) VALUES ('Carl', 10, 1);
INSERT INTO members(name, knockouts, team_id) VALUES ('Omera', 11, 1);


INSERT INTO teams(id, name) VALUES (2, 'Mercedes')
INSERT INTO members(name, knockouts, team_id) VALUES ('Marcus', 10, 2);
INSERT INTO members(name, knockouts, team_id) VALUEs ('masterchef', 2000, 2);