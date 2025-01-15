use crate::cfg::db_pg_cfg;
use crate::users::*;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    Error as PGSQLError, PgPool, Pool, Postgres,
};
use uuid::Uuid;
pub trait UsersManager {
    type Error: std::fmt::Display + std::error::Error;

    async fn register_new_user(&self, u: &User) -> Result<(), Self::Error>;
    async fn get_user_by_id(&self, id: u64) -> Result<User, Self::Error>;
}
pub trait SessionManager{
    type Error: std::fmt::Display + std::error::Error; 
    async fn register_new_session(
        &self,
        u: &User,
        uuid: Option<Uuid>,
    ) -> Result<Option<Uuid>, Self::Error>;
    async fn resolve_user_session(&self, uid: Uuid) 
    -> Result<Option<Uuid>, Self::Error>;
}
struct DBPostgres {
    pool: Pool<Postgres>,
}
impl DBPostgres {
    pub async fn try_init(cfg: &db_pg_cfg) -> Result<Self, Box<dyn std::error::Error>> {
        let mut pgconn_opt = PgConnectOptions::new()
            .application_name("tester_backend")
            .database(&cfg.db_name)
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
                Err(Box::new(e))    // TODO: Refactor error handling here.
                                    // TODO: Early check, change pack.
                                    // TODO: Consider using anyhow crate?
            },
        }
    }
}
 
impl UsersManager for DBPostgres { 
    type Error = PGSQLError;
    async fn register_new_user(&self, u: &User) -> Result<(), Self::Error> {
         let c = self.pool.acquire().await?;
            
        todo!()
    }
    async fn get_user_by_id(&self, id: u64) -> Result<User, Self::Error> {
        todo!()
    }
}
impl SessionManager for PGSQLError{
    type Error = PGSQLError;   
    async fn register_new_session(
            &self,
            u: &User,
            uuid: Option<Uuid>,
        ) -> Result<Option<Uuid>, Self::Error> {
        todo!()
    }
    async fn resolve_user_session(&self, uid: Uuid) 
        -> Result<Option<Uuid>, Self::Error> {
            todo!()
    }

}