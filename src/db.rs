use crate::models::dtos::UserRegisterReq;
use crate::cfg::DBPGCfg;
use crate::models::dtos;
use crate::models::users::*;
use crate::models::knowledge_test;
use async_trait::async_trait; // make traits with async dyn-compatable.
use passhash::hash_password;
use std::error::Error;
use sqlx::{
    query,postgres::{PgConnectOptions, PgPoolOptions},
    Error as PGSQLError, PgPool, Pool, Postgres,
};
use uuid::Uuid;

#[async_trait]
pub trait UsersManager {

    async fn register_new_user(&self, u: &UserRegisterReq) -> Result<(), Box<dyn Error>>;
    async fn get_user_by_id(&self, id: u64) -> Result<User, Box<dyn Error>>;
}
#[async_trait]
pub trait SessionManager{
    async fn register_new_session(
        &self,
        u: &User,
        uuid: Option<Uuid>,
    ) -> Result<Option<Uuid>, Box<dyn Error>>;
    async fn resolve_user_session(&self, uid: Uuid) 
    -> Result<Option<Uuid>,  Box<dyn Error>>;
}
pub struct DBPostgres {
    pool: Pool<Postgres>,
}
impl DBPostgres {
    pub async fn try_init(cfg: &DBPGCfg) -> Result<Self, Box<dyn std::error::Error>> {
        let mut pgconn_opt = PgConnectOptions::new()
            .application_name("tester_backend")
            .database(&cfg.db_name)
            .username("postgres")
            .host(&cfg.host)
            .port(if let Some(portf) = cfg.port {
                portf // Port was provided in config
            } else {
                5432 // default postgres port.
            });
        match PgPoolOptions::new()
            .max_connections(5)
            .connect_with(pgconn_opt)
            .await
        {
            Ok(r) => Ok(Self { pool: r }),
            Err(e) => {
                Err(Box::new(e)) // TODO: Refactor error handling here.
                                 // TODO: Early check, change pack.
                                 // TODO: Consider using anyhow crate?
            }
        }
    }
}
#[async_trait]
impl UsersManager for DBPostgres {
    async fn register_new_user(&self, u: &dtos::UserRegisterReq) -> Result<(), Box<dyn Error> > {
        let mut trx = self.pool.begin().await?;
        let pass_hash = hash_password(&u.password)?;
        let q = query!(r#"INSERT INTO users (login, password_hash, first_name,second_name, last_name,
         asigned_groups, asigned_role )
        VALUES ($1,$2,$3,$4,$5,$6,($7::text)::user_role  ) 
         "#,
        u.login, pass_hash,
        u.first_name, u.seocnd_name, u.last_name,
        u.groups_asigned.as_deref(), u.role_asigned  
         );
          
        todo!()
    }
    async fn get_user_by_id(&self, id: u64) -> Result<User, Box<dyn Error>> {
        todo!()
    }
}
#[async_trait]
impl SessionManager for DBPostgres {
    async fn register_new_session(
        &self,
        u: &User,
        uuid: Option<Uuid>,
    ) -> Result<Option<Uuid>, Box<dyn Error>> {
        todo!()
    }
    async fn resolve_user_session(&self, uid: Uuid) -> Result<Option<Uuid>, Box<dyn Error> > {
        todo!()
    }
}
