use crate::prelude::{Result, *};
mod sql;
use sql::*;

use sqlx::mysql::MySqlPool;

use crate::user::roles::Roles;
use sqlx::encode::IsNull;
use sqlx::error::BoxDynError;
use sqlx::*;

impl Type<MySql> for Roles {
    fn type_info() -> <MySql as Database>::TypeInfo {
        <[u8] as Type<MySql>>::type_info()
    }
}

impl<'q> Encode<'q, MySql> for Roles {
    fn encode_by_ref(
        &self,
        buf: &mut <MySql as Database>::ArgumentBuffer<'q>,
    ) -> std::result::Result<IsNull, BoxDynError> {
        let bytes = bson::to_vec(self)?;
        <&[u8] as Encode<MySql>>::encode_by_ref(&bytes.as_slice(), buf)
    }
}

impl<'q> Decode<'q, MySql> for Roles {
    fn decode(value: <MySql as Database>::ValueRef<'q>) -> std::result::Result<Self, BoxDynError> {
        let bytes = <&[u8] as Decode<MySql>>::decode(value)?;
        Ok(bson::from_slice(bytes)?)
    }
}

#[rocket::async_trait]
impl DBConnection for MySqlPool {
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
            .bind(&user.email)
            .bind(&user.password)
            .bind(bson::to_vec(&user.roles).unwrap())
            .bind(user.id)
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
}
