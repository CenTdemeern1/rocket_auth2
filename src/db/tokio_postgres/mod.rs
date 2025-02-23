use crate::prelude::*;
mod sql;
use crate::user::roles::Roles;
use std::convert::{TryFrom, TryInto};
use tokio_postgres::types::private::BytesMut;
use tokio_postgres::types::{FromSql, IsNull, ToSql, Type};
use tokio_postgres::Client;

impl ToSql for Roles {
    fn to_sql(
        &self,
        ty: &Type,
        out: &mut BytesMut,
    ) -> std::result::Result<IsNull, Box<dyn std::error::Error + Sync + Send>>
    where
        Self: Sized,
    {
        let bytes = bson::to_vec(self)?;
        <Vec<u8> as ToSql>::to_sql(&bytes, ty, out)
    }

    fn accepts(ty: &Type) -> bool
    where
        Self: Sized,
    {
        <Vec<u8> as ToSql>::accepts(ty)
    }

    fn to_sql_checked(
        &self,
        ty: &Type,
        out: &mut BytesMut,
    ) -> std::result::Result<IsNull, Box<dyn std::error::Error + Sync + Send>> {
        let bytes = bson::to_vec(self)?;
        <Vec<u8> as ToSql>::to_sql(&bytes, ty, out)
    }
}

impl<'a> FromSql<'a> for Roles {
    fn from_sql(
        ty: &Type,
        raw: &'a [u8],
    ) -> std::result::Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        let bytes = <Vec<u8> as FromSql>::from_sql(ty, raw)?;
        Ok(bson::from_slice(&bytes)?)
    }

    fn accepts(ty: &Type) -> bool {
        <Vec<u8> as FromSql>::accepts(ty)
    }
}

#[rocket::async_trait]
impl DBConnection for Client {
    async fn init(&self) -> Result<()> {
        self.execute(sql::CREATE_TABLE, &[]).await?;
        Ok(())
    }
    async fn create_user(&self, email: &str, hash: &str, roles: &Roles) -> Result<(), Error> {
        self.execute(sql::INSERT_USER, &[&email, &hash, roles])
            .await?;
        Ok(())
    }
    async fn update_user(&self, user: &User) -> Result<()> {
        self.execute(
            sql::UPDATE_USER,
            &[&user.email, &user.password, &user.roles],
        )
        .await?;
        Ok(())
    }
    async fn delete_user_by_id(&self, user_id: i32) -> Result<()> {
        self.execute(sql::REMOVE_BY_ID, &[&user_id]).await?;
        Ok(())
    }
    async fn delete_user_by_email(&self, email: &str) -> Result<()> {
        self.execute(sql::REMOVE_BY_EMAIL, &[&email]).await?;
        Ok(())
    }
    async fn get_user_by_id(&self, user_id: i32) -> Result<User> {
        let user = self.query_one(sql::SELECT_BY_ID, &[&user_id]).await?;
        user.try_into()
    }

    async fn get_user_by_email(&self, email: &str) -> Result<User> {
        let user = self.query_one(sql::SELECT_BY_EMAIL, &[&email]).await?;
        user.try_into()
    }

    async fn get_all_ids(&self) -> Result<Vec<i32>> {
        let rows = self.query(sql::GET_ALL, &[]).await?;
        let ids = rows
            .into_iter()
            .map(|row| row.get::<usize, i32>(0))
            .collect();
        Ok(ids)
    }
}

impl TryFrom<tokio_postgres::Row> for User {
    type Error = Error;
    fn try_from(row: tokio_postgres::Row) -> Result<User> {
        Ok(User {
            id: row.get(0),
            email: row.get(1),
            password: row.get(2),
            roles: row.get(3),
        })
    }
}
