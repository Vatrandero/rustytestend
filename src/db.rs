use std::fmt::Display;

use crate::cfg::db_pg_cfg;
use crate::users::*;
use sqlx::{Error as sqlError, Pool, Postgres};
use uuid::Uuid;
pub trait UsersAndSessionManager {
    type Error: Display + std::error::Error; 

    async fn register_new_user(&self, u: &User) -> Result<(), Self::Error>;
    async fn get_user_by_id(&self, id: u64 ) -> Result<User, Self::Error >;
    async fn register_new_session(
        &self,
        u: &User,
        uuid: Option<Uuid>,
    ) -> Result<Option<Uuid>, Self::Error>;
    async fn resolve_user_session(&self, uid: Uuid) 
    -> Result<Option<Uuid>, Self::Error>;

}

struct DBPostgres {}
