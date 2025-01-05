//! This module contains
//! code needed to
//! communicate with user
//! via HTTP/REST apj and keeping connections.

use tokio::net::*;
use warp::*;
pub fn init(cfg:crate::cfg::Config){ 

}

mod user_manage { 

    pub fn create_user() -> Result<(), ()> { 
        Ok(())
    }
}
mod session_managment{}