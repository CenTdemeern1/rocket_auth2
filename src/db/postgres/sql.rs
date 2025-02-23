pub(crate) const CREATE_TABLE: &str = "
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    email VARCHAR (254) UNIQUE NOT NULL,
	password VARCHAR ( 255 ) NOT NULL,
    roles BYTEA NOT NULL
);
";

pub(crate) const INSERT_USER: &str = "
INSERT INTO users (email, password, roles) VALUES ($1, $2, $3);
";

pub(crate) const UPDATE_USER: &str = "
UPDATE users SET
    email = $2,
    password = $3,
    roles = $4
WHERE
    id = $1
";

pub(crate) const SELECT_BY_ID: &str = "
SELECT * FROM users WHERE id = $1;
";

pub(crate) const SELECT_BY_EMAIL: &str = "
SELECT * FROM users WHERE email = $1;
";

pub(crate) const REMOVE_BY_ID: &str = "
DELETE FROM users WHERE id =$1;
";
pub(crate) const REMOVE_BY_EMAIL: &str = "
DELETE FROM users WHERE email =$1;
";
pub(crate) const GET_ALL: &str = "
SELECT id FROM users;
";
