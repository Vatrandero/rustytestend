/// REST Api.
use crate::models::dtos;
pub(super) mod routes;

use crate::db::{SessionManager, UsersManager};
use crate::models::users::User;
use std::sync::Arc;
use axum::{Router, routing::{}};
#[cfg(feature = "apidoc")]
use utoipa::openapi::OpenApi;

#[derive(Clone)]
pub struct AppState {
    pub dbpool_user_manager: Arc<dyn UsersManager + Send + Sync>,
    pub dbpool_session_manager: Arc<dyn SessionManager + Send + Sync>,
    /* Why using dyn?
    In future, there considired multi-db mode or
    db+cache (like redis) modes.
    If only db provided - both Arcs needs to
    be directedd at the same dbpool.
    This text shall be removed when feature will
    be implemented.
     */
}

/// Based on passed configs 
pub fn init_router(cfg: &crate::cfg::Config) -> axum::Router<AppState> {
    let mut r: Router<AppState> = Router::new();


    todo!()
}

#[cfg(feature="apidoc")]
pub fn route_openapi() -> Router {
    todo!()
}
