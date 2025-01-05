use crate::cfg::db_pg_cfg;
use sqlx::{Pool, Postgres}; 

pub trait SQLDBUserManager {} { 
    pub fn create_user(&self) ->  Result<(), sqlx::Error>;
        
}


