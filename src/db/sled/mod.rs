use crate::prelude::*;
use crate::user::roles::Roles;
use sled::transaction::{ConflictableTransactionError, ConflictableTransactionResult};
use sled::Transactional;

const TABLE_NAME: &str = "users";
const EMAIL_INDEX_NAME: &str = "users_emails";

#[derive(Deserialize, Serialize)]
struct UserData {
    email: String,
    hash: String,
    roles: Roles,
}

fn map_error(e: impl Into<Error>) -> ConflictableTransactionError<Error> {
    ConflictableTransactionError::Abort(e.into())
}

fn serialize_id(id: i32) -> [u8; size_of::<i32>()] {
    id.to_be_bytes()
}

fn deserialize_id(id: &[u8]) -> i32 {
    i32::from_be_bytes(id[..].try_into().unwrap())
}

fn serialize_email(email: &str) -> &[u8] {
    email.as_bytes()
}

fn serialize_data(data: &UserData) -> Vec<u8> {
    bson::to_vec(data).unwrap()
}

fn deserialize_data(data: &[u8]) -> UserData {
    bson::from_slice(data).unwrap()
}

#[rocket::async_trait]
impl DBConnection for sled::Db {
    async fn init(&self) -> Result<()> {
        self.open_tree(TABLE_NAME)?;
        self.open_tree(EMAIL_INDEX_NAME)?;
        Ok(())
    }

    async fn create_user(&self, email: &str, hash: &str, roles: &Roles) -> Result<()> {
        let id: i32 = self.generate_id()? as i32;
        let tree = self.open_tree(TABLE_NAME)?;
        let index = self.open_tree(TABLE_NAME)?;

        (&tree, &index).transaction(
            |(tree, index)| -> ConflictableTransactionResult<(), Error> {
                index.insert(serialize_email(email), &serialize_id(id))?;

                let data = UserData {
                    email: email.to_string(),
                    hash: hash.to_string(),
                    roles: roles.clone(),
                };
                tree.insert(&serialize_id(id), serialize_data(&data))?;

                Ok(())
            },
        )?;

        Ok(())
    }

    async fn update_user(&self, user: &User) -> Result<()> {
        let tree = self.open_tree(TABLE_NAME)?;
        let index = self.open_tree(TABLE_NAME)?;

        (&tree, &index).transaction(
            |(tree, index)| -> ConflictableTransactionResult<(), Error> {
                let data = UserData {
                    email: user.email.clone(),
                    hash: user.password.clone(),
                    roles: user.roles.clone(),
                };

                let old_entry = tree.insert(&serialize_id(user.id), serialize_data(&data))?;

                if let Some(old_entry) = old_entry {
                    let old_user = deserialize_data(&old_entry);
                    index.remove(serialize_email(&old_user.email))?;
                }

                index.insert(serialize_email(&user.email), &serialize_id(user.id))?;

                Ok(())
            },
        )?;

        Ok(())
    }

    async fn delete_user_by_id(&self, user_id: i32) -> Result<()> {
        let tree = self.open_tree(TABLE_NAME)?;
        let index = self.open_tree(TABLE_NAME)?;

        (&tree, &index).transaction(
            |(tree, index)| -> ConflictableTransactionResult<(), Error> {
                let old_entry = tree.remove(&serialize_id(user_id))?;

                if let Some(old_entry) = old_entry {
                    let old_user = deserialize_data(&old_entry);
                    index.remove(serialize_email(&old_user.email))?;
                }

                Ok(())
            },
        )?;

        Ok(())
    }

    async fn delete_user_by_email(&self, email: &str) -> Result<()> {
        let tree = self.open_tree(TABLE_NAME)?;
        let index = self.open_tree(TABLE_NAME)?;

        (&tree, &index).transaction(
            |(tree, index)| -> ConflictableTransactionResult<(), Error> {
                let old_entry = index.remove(serialize_email(email))?;
                if let Some(old_entry) = old_entry {
                    tree.remove(old_entry)?;
                }

                Ok(())
            },
        )?;

        Ok(())
    }

    async fn get_user_by_id(&self, user_id: i32) -> Result<User> {
        let tree = self.open_tree(TABLE_NAME)?;

        let user = tree
            .get(serialize_id(user_id))?
            .ok_or(Error::UserNotFoundError)?;

        let user = deserialize_data(&user);

        Ok(User {
            id: user_id,
            email: user.email,
            roles: user.roles,
            password: user.hash,
        })
    }

    async fn get_user_by_email(&self, email: &str) -> Result<User> {
        let tree = self.open_tree(TABLE_NAME)?;
        let index = self.open_tree(TABLE_NAME)?;

        let user = (&tree, &index).transaction(
            |(tree, index)| -> ConflictableTransactionResult<User, Error> {
                let id = index
                    .get(serialize_email(email))?
                    .ok_or(Error::UserNotFoundError)
                    .map_err(map_error)?;
                let user = tree.get(&id)?.unwrap();

                let user = deserialize_data(&user);

                Ok(User {
                    id: deserialize_id(&id),
                    email: user.email,
                    roles: user.roles,
                    password: user.hash,
                })
            },
        )?;

        Ok(user)
    }
}
