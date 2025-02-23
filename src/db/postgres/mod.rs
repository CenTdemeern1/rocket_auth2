use crate::prelude::{Result, *};
mod sql;
use sql::*;

use sqlx::postgres::PgPool;

use crate::user::roles::Roles;
use sqlx::encode::IsNull;
use sqlx::error::BoxDynError;
use sqlx::*;

impl Type<Postgres> for Roles {
    fn type_info() -> <Postgres as Database>::TypeInfo {
        <[u8] as Type<Postgres>>::type_info()
    }
}

impl<'q> Encode<'q, Postgres> for Roles {
    fn encode_by_ref(
        &self,
        buf: &mut <Postgres as Database>::ArgumentBuffer<'q>,
    ) -> std::result::Result<IsNull, BoxDynError> {
        let bytes = bson::to_vec(self)?;
        <&[u8] as Encode<Postgres>>::encode_by_ref(&bytes.as_slice(), buf)
    }
}

impl<'q> Decode<'q, Postgres> for Roles {
    fn decode(
        value: <Postgres as Database>::ValueRef<'q>,
    ) -> std::result::Result<Self, BoxDynError> {
        let bytes = <&[u8] as Decode<Postgres>>::decode(value)?;
        Ok(bson::from_slice(bytes)?)
    }
}
#[rocket::async_trait]
impl DBConnection for PgPool {
    async fn init(&self) -> Result<()> {
        query(CREATE_TABLE).execute(self).await?;
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
        query(REMOVE_BY_ID).bind(user_id).execute(self).await?;
        Ok(())
    }
    async fn delete_user_by_email(&self, email: &str) -> Result<()> {
        query(REMOVE_BY_EMAIL).bind(email).execute(self).await?;
        Ok(())
    }
    async fn get_user_by_id(&self, user_id: i32) -> Result<User> {
        let user = query_as(SELECT_BY_ID).bind(user_id).fetch_one(self).await?;

        Ok(user)
    }
    async fn get_user_by_email(&self, email: &str) -> Result<User> {
        let user = query_as(SELECT_BY_EMAIL)
            .bind(email)
            .fetch_one(self)
            .await?;
        Ok(user)
    }
    async fn get_all_ids(&self) -> Result<Vec<i32>> {
        let ids = query_scalar(GET_ALL).fetch_all(self).await?;
        Ok(ids)
    }
}
