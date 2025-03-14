mod sql;

use crate::prelude::{Result, *};
use crate::user::roles::Roles;
use rocket::async_trait;
use sql::*;
use std::borrow::Cow;
use tokio::sync::Mutex;

#[cfg(feature = "rusqlite")]
use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput};
#[cfg(feature = "rusqlite")]
use rusqlite::Row;
#[cfg(feature = "rusqlite")]
use rusqlite::*;
#[cfg(feature = "rusqlite")]
use std::convert::{TryFrom, TryInto};
#[cfg(feature = "rusqlite")]
use tokio::task::block_in_place;

#[cfg(feature = "rusqlite")]
impl FromSql for Roles {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> FromSqlResult<Self> {
        match value {
            rusqlite::types::ValueRef::Blob(bytes) => {
                Ok(bson::from_slice(bytes).map_err(|e| FromSqlError::Other(Box::new(e)))?)
            }
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

#[cfg(feature = "rusqlite")]
impl ToSql for Roles {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        let bytes = bson::to_vec(self).map_err(|e| FromSqlError::Other(Box::new(e)))?;
        Ok(bytes.into())
    }
}

#[cfg(feature = "rusqlite")]
impl<'a> TryFrom<&rusqlite::Row<'a>> for crate::User {
    type Error = rusqlite::Error;
    fn try_from(row: &Row) -> Result<User, rusqlite::Error> {
        Ok(User {
            id: row.get(0)?,
            email: row.get(1)?,
            password: row.get(2)?,
            roles: row.get(3)?,
        })
    }
}

#[cfg(feature = "rusqlite")]
#[async_trait]
impl DBConnection for Mutex<rusqlite::Connection> {
    async fn init(&self) -> Result<()> {
        let conn = self.lock().await;
        block_in_place(|| conn.execute(CREATE_TABLE, []))?;
        Ok(())
    }

    async fn create_user(&self, email: &str, hash: &str, roles: &Roles) -> Result<()> {
        let conn = self.lock().await;
        block_in_place(|| conn.execute(INSERT_USER, params![email, hash, roles]))?;

        Ok(())
    }

    async fn update_user(&self, user: &User) -> Result<()> {
        let conn = self.lock().await;
        block_in_place(|| {
            conn.execute(
                UPDATE_USER,
                params![user.id, user.email, user.password, user.roles],
            )
        })?;
        Ok(())
    }

    async fn delete_user_by_id(&self, user_id: i32) -> Result<()> {
        let conn = self.lock().await;
        block_in_place(|| conn.execute(REMOVE_BY_ID, params![user_id]))?;
        Ok(())
    }

    async fn delete_user_by_email(&self, email: &str) -> Result<()> {
        let conn = self.lock().await;
        block_in_place(|| conn.execute(REMOVE_BY_EMAIL, params![email]))?;
        Ok(())
    }

    async fn get_user_by_id(&self, user_id: i32) -> Result<User> {
        let conn = self.lock().await;
        let user = block_in_place(|| {
            conn.query_row(
                SELECT_BY_ID, //
                params![user_id],
                |row| row.try_into(),
            )
        })?;
        Ok(user)
    }

    async fn get_user_by_email(&self, email: &str) -> Result<User> {
        let conn = self.lock().await;
        let user = block_in_place(|| {
            conn.query_row(
                SELECT_BY_EMAIL, //
                params![email],
                |row| row.try_into(),
            )
        })?;
        Ok(user)
    }
    async fn get_all_ids(&self) -> Result<Vec<i32>> {
        let conn = self.lock().await;
        let mut stmt = conn.prepare(GET_ALL)?;
        let ids = block_in_place(|| -> Result<Vec<i32>> {
            Ok(stmt
                .query_map([], |row| row.get::<usize, i32>(0))?
                .flatten()
                .collect())
        })?;
        Ok(ids)
    }
}

#[cfg(feature = "sqlx-sqlite")]
use sqlx::encode::IsNull;
#[cfg(feature = "sqlx-sqlite")]
use sqlx::error::BoxDynError;
use sqlx::sqlite::SqliteArgumentValue;
#[cfg(feature = "sqlx-sqlite")]
use sqlx::{sqlite::SqliteConnection, *};

#[cfg(feature = "sqlx-sqlite")]
impl Type<Sqlite> for Roles {
    fn type_info() -> <Sqlite as Database>::TypeInfo {
        <[u8] as Type<Sqlite>>::type_info()
    }
}

#[cfg(feature = "sqlx-sqlite")]
impl<'q> Encode<'q, Sqlite> for Roles {
    fn encode_by_ref(
        &self,
        buf: &mut <Sqlite as Database>::ArgumentBuffer<'q>,
    ) -> std::result::Result<IsNull, BoxDynError> {
        let bytes = bson::to_vec(self)?;
        buf.push(SqliteArgumentValue::Blob(Cow::Owned(bytes)));
        Ok(IsNull::No)
    }
}

#[cfg(feature = "sqlx-sqlite")]
impl<'q> Decode<'q, Sqlite> for Roles {
    fn decode(value: <Sqlite as Database>::ValueRef<'q>) -> std::result::Result<Self, BoxDynError> {
        let bytes = <&[u8] as Decode<Sqlite>>::decode(value)?;
        Ok(bson::from_slice(bytes)?)
    }
}

#[cfg(feature = "sqlx-sqlite")]
#[async_trait]
impl DBConnection for Mutex<SqliteConnection> {
    async fn init(&self) -> Result<()> {
        let mut db = self.lock().await;
        query(CREATE_TABLE).execute(&mut *db).await?;
        println!("table created");
        Ok(())
    }
    async fn create_user(&self, email: &str, hash: &str, roles: &Roles) -> Result<()> {
        let mut db = self.lock().await;
        query(INSERT_USER)
            .bind(email)
            .bind(hash)
            .bind(roles)
            .execute(&mut *db)
            .await?;
        Ok(())
    }
    async fn update_user(&self, user: &User) -> Result<()> {
        let mut db = self.lock().await;
        query(UPDATE_USER)
            .bind(user.id)
            .bind(&user.email)
            .bind(&user.password)
            .bind(&user.roles)
            .execute(&mut *db)
            .await?;
        Ok(())
    }
    async fn delete_user_by_id(&self, user_id: i32) -> Result<()> {
        query(REMOVE_BY_ID)
            .bind(user_id)
            .execute(&mut *self.lock().await)
            .await?;
        Ok(())
    }
    async fn delete_user_by_email(&self, email: &str) -> Result<()> {
        query(REMOVE_BY_EMAIL)
            .bind(email)
            .execute(&mut *self.lock().await)
            .await?;
        Ok(())
    }
    async fn get_user_by_id(&self, user_id: i32) -> Result<User> {
        let mut db = self.lock().await;

        let user = query_as(SELECT_BY_ID)
            .bind(user_id)
            .fetch_one(&mut *db)
            .await?;

        Ok(user)
    }
    async fn get_user_by_email(&self, email: &str) -> Result<User> {
        let mut db = self.lock().await;
        let user = query_as(SELECT_BY_EMAIL)
            .bind(email)
            .fetch_one(&mut *db)
            .await?;
        Ok(user)
    }
    async fn get_all_ids(&self) -> Result<Vec<i32>> {
        let mut db = self.lock().await;
        let ids = query_scalar(GET_ALL).fetch_all(&mut *db).await?;
        Ok(ids)
    }
}
#[cfg(feature = "sqlx-sqlite")]
#[rocket::async_trait]
impl DBConnection for SqlitePool {
    async fn init(&self) -> Result<()> {
        query(CREATE_TABLE) //
            .execute(self)
            .await?;
        Ok(())
    }
    async fn create_user(&self, email: &str, hash: &str, roles: &Roles) -> Result<()> {
        query(INSERT_USER)
            .bind(email)
            .bind(hash)
            .bind(roles)
            .execute(self)
            .await?;
        Ok(())
    }
    async fn update_user(&self, user: &User) -> Result<()> {
        query(UPDATE_USER)
            .bind(user.id)
            .bind(&user.email)
            .bind(&user.password)
            .bind(&user.roles)
            .execute(self)
            .await?;
        Ok(())
    }
    async fn delete_user_by_id(&self, user_id: i32) -> Result<()> {
        query(REMOVE_BY_ID) //
            .bind(user_id)
            .execute(self)
            .await?;
        Ok(())
    }
    async fn delete_user_by_email(&self, email: &str) -> Result<()> {
        query(REMOVE_BY_EMAIL) //
            .bind(email)
            .execute(self)
            .await?;
        Ok(())
    }
    async fn get_user_by_id(&self, user_id: i32) -> Result<User> {
        let user = query_as(SELECT_BY_ID) //
            .bind(user_id)
            .fetch_one(self)
            .await?;
        Ok(user)
    }
    async fn get_user_by_email(&self, email: &str) -> Result<User> {
        let user = query_as(SELECT_BY_EMAIL).bind(email).fetch_one(self).await;
        println!("user: {:?}", user);
        Ok(user?)
    }
    async fn get_all_ids(&self) -> Result<Vec<i32>> {
        let ids = query_scalar(GET_ALL).fetch_all(self).await?;
        Ok(ids)
    }
}
