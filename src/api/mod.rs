/// REST Api.
use crate::models::dtos;
pub mod routes;

use crate::models::users::User;
use crate::db::{UsersManager, SessionManager};
use std::sync::Arc;
use tokio::net::*;
use utoipa::openapi::OpenApi;


pub fn init(cfg: crate::cfg::Config) {}

pub fn generate_api_spec() -> OpenApi {
    todo!()
}

pub(super) struct DataPool {
    pub dbpool_user_manager: Arc<dyn  UsersManager>, 
    pub dbpool_session_manager: Arc<dyn SessionManager>                                                   
}