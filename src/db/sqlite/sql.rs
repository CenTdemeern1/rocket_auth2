pub(crate) const CREATE_TABLE: &str = "
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY,
    email TEXT UNIQUE,
    password TEXT NOT NULL,
    roles BLOB NOT NULL
    -- failed_login_attempts INTEGER DEFAULT 0

);";

pub(crate) const INSERT_USER: &str = "
INSERT INTO users (email, password, roles) VALUES (?1, ?2, ?3);
";

pub(crate) const UPDATE_USER: &str = "
UPDATE users SET 
    email = ?2,
    password = ?3,
    roles = ?4
WHERE
    id = ?1;
";

pub(crate) const SELECT_BY_ID: &str = "
SELECT * FROM users WHERE id = ?1;
";

pub(crate) const SELECT_BY_EMAIL: &str = "
SELECT * FROM users WHERE email = ?1;
";

pub(crate) const REMOVE_BY_ID: &str = "
DELETE FROM users WHERE id =?1;
";
pub(crate) const REMOVE_BY_EMAIL: &str = "
DELETE FROM users WHERE email =?1;
";
