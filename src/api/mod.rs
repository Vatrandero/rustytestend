/// REST Api.
use crate::models::dtos;
pub(super) mod routes;

use crate::db::{SessionManager, UsersManager};
use crate::models::users::User;
use std::sync::Arc;
use axum::{Router, routing::{get, post, put}};
#[cfg(feature = "apidoc")]
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    
)]
pub struct Apidoc; 

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
pub fn init_router(cfg: &crate::cfg::Config, state: AppState) -> axum::Router {
    let mut r: Router<AppState> = Router::new();
    r = r.route("/usrmngr/register", post(routes::admin_handlers::register));
    #[cfg(feature="apidoc")]   
    if cfg.api_cfg.host_doc { 
        r = r.nest("/doc",builded_openapi_for_router() );
        info!("OpenApi doc & swagger is included in routes!")
    }
    
    r.with_state(state)
}

/// Geneerates handlers with documentation for given
/// Router.
/// 
#[cfg(feature="apidoc")]
pub fn builded_openapi_for_router() -> Router<AppState> {
    let openapi = Apidoc::openapi();
    let mut router: Router<AppState> = Router::new();//FIXME: We don;t realy need state here.      
    let sjson = openapi.to_json().unwrap();
    let syaml = openapi.to_yaml().unwrap();
   // let swagger  utoipa_swagger_ui::SwaggerUi::
    
    router
    .route("/schema.json", get(|| async{sjson}))
    .route("/schema.yaml", get(|| async{syaml}))
    .merge(utoipa_swagger_ui::SwaggerUi
        ::new("/swagger-ui")) 
    // TODO: Swagger should we feature.

}
