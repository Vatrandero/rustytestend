use crate::cfg::DBPGCfg;
use crate::models;
use crate::models::dtos;
use crate::models::dtos::UserRegisterReq;
use crate::models::users::*;
use async_trait::async_trait; // make traits with async dyn-compatable.
use passhash::hash_password;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    query, query_as, Error as PGSQLError, PgPool, Pool, Postgres,
};
use std::{boxed::Box, fmt::format};
use std::error::Error;
use tokio::stream::*;
use uuid::Uuid;
/*
    Q: Why there are so many traits?
    A: Considered that for different data - will be used different DMBS.
    But it will b enden by using Postgressql for moost of data -
    and using redis for sessions.

*/
#[derive(Debug)]
pub enum DBError {
    RegisterSameUser(String),
    DBIOError(String),
    DBDataError(String),
    DBDataNotExisted(String),
    OtherErr(Box<dyn Error>)    
}
impl std::fmt::Display for DBError {
    fn fmt(&self, f: &mut  std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Error for DBError {}


impl From<Box<dyn Error>> for DBError { 
    fn from(val: Box<dyn Error>) -> DBError {
        DBError::OtherErr(val)    
    }
}


#[async_trait]
pub trait UsersManager {
    async fn register_new_user(&self, u: &UserRegisterReq) -> Result<(), Box<dyn Error>>;
    async fn get_user_by_id(&self, id: i32) -> Result<Option<User>, Box<dyn Error>>;

}
#[async_trait]
pub trait UsersSessionManager {
    async fn register_new_session(
        &self,
        u: &User,
    ) -> Result<Uuid, Box<dyn Error>>;
    /// Resolves users session cookie Uuid into user id.
    async fn resolve_user_session_to_id(&self, uuid: Uuid) -> Result<Option<i32>, Box<dyn Error>>; // i64?

    /// Uses if user unlogined.
    async fn end_session(&self, id: i32) -> Result<(), Box<dyn Error>>;
}

#[async_trait]
pub trait KTestManager {
    async fn nwe(&self);
    async fn list(&self, id: i64);
    async fn delete(&self, test_id: i64);

    async fn asign(&self, asign: models::dtos::UnAsignReq);
    async fn get_asign_by_ids(
        &self,
        user_id: i64,
        test_id: i64,
    ) -> Result<models::knowledge_test::KtAsigment, Box<dyn Error>>;

    /// this method needs to be called when starting new KtESTsESSION.
    /// decrease tries for given asigment in DB
    async fn decrease_asignment(&self, asign: models::knowledge_test::KtAsigment);

    async fn unasign(&self, unasign: models::dtos::UnAsignReq);
}
#[async_trait]
pub trait KTestSessionManager {
    /// Tries for given asigment run the new test session.
    async fn new(
        &self,
        asign: models::knowledge_test::KtAsigment,
    ) -> Result<models::knowledge_test::KTestOngoing, Box<dyn Error>>;

    #[deprecated = "This method exists because of problem 
    occured on traits design.
    See doc comment for explanation."]
    /// Cancels given test session, deletes
    /// session without traces.
    /// Use this method to rollback bad session.
    /// This is not same as `end_session`, wich just marks
    /// session ended, completed.
    ///
    /// # DEPRECATED Reason:
    /// When designed this KTEstSessionManager, in considered
    /// that this may different DBMS (Redis for caching, as example)
    /// There are some solutions usable.
    ///
    /// One solutionn is to make this method accept `&dyn KTestManager`
    /// and call it's methods from here.
    /// Other solution is to merge traits.
    /// And  another curretnly used - to fully seperate
    /// traits and make caller be responsible for tracking
    /// both managers acted succusuflly.
    ///
    /// Tha last solution: Create one more entity, that will be 
    /// controlling transaction of session starting.
    /// If it nas started - commit, if not - rollback.
    /// 
    /// In all ways, seems like we also should use UUID instead of INTEGER.
    ///    
    async fn cancel(&self);

    /// Takes updated `KTestOngoing`, considered
    /// it has modifications by user.
    /// Yhis method valodates original test data and tries
    /// to commit changes.
    async fn update(&self, ko: models::knowledge_test::KTestOngoing) -> Result<(), Box<dyn Error>>;
    /// Succusuflly end sesion.
    async fn end_session(&self, id: i64) -> Result<(), Box<dyn Error>>;
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
    async fn register_new_user(&self, u: &dtos::UserRegisterReq) -> Result<(), Box<dyn Error>> {
        let mut exe = self.pool.begin().await?;
                
        // We already have such user?
        let check = query!(r#"SELECT login FROM users WHERE login = ($1)"#, 
        &u.login).fetch_optional(&mut *exe).await?;
        if let Some(_) = check { 
            return Err("Tried to register existed user.".into())
             }
        // Register
        let pass_hash = hash_password(&u.password)?;
        let q = query!(
            r#"INSERT INTO users (login, password_hash,
            first_name,second_name, last_name,
            asigned_groups, asigned_role )
            VALUES ($1,$2,$3,$4,$5,$6,($7::text)::user_role  ) 
        "#,
            u.login,
            pass_hash,
            u.first_name,
            u.seocnd_name,
            u.last_name,
            u.groups_asigned.as_deref(),
            u.role_asigned
        );

        q.execute(&mut *exe).await?;

        exe.commit().await?;
        Ok(())
    }
    async fn get_user_by_id(&self, id: i32) -> Result<Option<User>, Box<dyn Error>> {
        let mut exe = self.pool.acquire().await?;
        let r = query_as!(
            models::users::User,
             r#"SELECT
             id, first_name, second_name, last_name, 
             asigned_role as "user_role: UserRole" , login, password_hash 
             FROM users WHERE id = $1 "#, id)
            .fetch_one(&mut *exe).await;
        
        match r { 
            Ok(o) => Ok(Some(o)),
            Err(e) => {
                match e {
                    PGSQLError::RowNotFound => Ok(None), 
                    _ => Err(Box::new(e))
                }
            }  
        }
    }
}
#[async_trait]
impl UsersSessionManager for DBPostgres {
    async fn register_new_session(
        &self,
        u: &User,
    ) -> Result<Uuid, Box<dyn Error>> {
        let mut exe = self.pool.begin().await?; 
        query!(r#"INSERT INTO user_sessions (user_id, created_at, expires_at)
        VALUES ($1, $2, $3) "#,
        u.id,
        chrono::Utc::now().naive_utc(),
        chrono::Utc::now().naive_utc() + std::time::Duration::from_secs(604800) // 7 days..
      ).execute(&mut *exe).await?; 
        exe.commit().await?;

        // Done, tyu to extract generated UUID
        Ok(query!("SELECT sid FROM user_sessions WHERE user_id = $1 ", u.id)
        .fetch_one(&mut *(self.pool.acquire().await?)).await?.sid)
    }
    async fn resolve_user_session_to_id(&self, uuid: Uuid) 
    -> Result<Option<i32>, Box<dyn Error>> {
        let mut exe = self.pool.acquire().await?;
        let mut  r = query!(r#"SELECT user_id FROM user_sessions
        WHERE sid = $1 "#, uuid ).fetch_one(&mut *exe).await;        
        let mut encountered = false;
        match r {
            Ok(o) => {Ok(Some(o.user_id))},
            Err(e) => { 
                match e{
                    PGSQLError::RowNotFound => Ok(None),
                    _ => Err(e.into()) // TODO: Match more errors?
                    
                }
                
            }
        }
        
        
        

    }

    async fn end_session(&self, id: i32) -> Result<(), Box<dyn Error>> {
        let mut exe = self.pool.begin().await?;
        // We will just drop all expired sessions.
        // TODO: Consider lock with `locked` column insteam.
        let r = query!("DELETE FROM user_sessions WHERE user_id = $1", id)
        .execute(&mut *exe).await?;
        exe.commit().await?;
        Ok(())        
    }
}
