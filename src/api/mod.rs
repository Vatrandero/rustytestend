/// REST Api.
pub(super) mod routes;
pub mod error;
use error::ApiError;
use crate::db::{UsersSessionManager, UsersManager};
use std::sync::Arc;
use axum::{Router, routing::{get, post, put}};
use utoipa::OpenApi;
use utoipauto::utoipauto;
/// Apidoc - placeholder, filled with 
/// outout of utoipa.
#[cfg(feature="apidoc")]
#[utoipauto] // TODO: Remove utoiauto and fill manualy.
#[derive(OpenApi)]
#[openapi(
    
)]
pub struct Apidoc; 

#[derive(Clone)]
pub struct AppState {
    pub dbpool_user_manager: Arc<dyn UsersManager + Send + Sync>,
    pub dbpool_session_manager: Arc<dyn UsersSessionManager + Send + Sync>,
    /* Why using dyn?
    In future, there considired multi-db mode or
    db+cache (like redis) modes.
    If only db provided - both Arcs needs to
    be directedd at the same dbpool.
    This text shall be removed when feature will
    be implemented.
     */
}

impl AppState { 
    /// Checks if user existed for given session id.
    /// Return None if no `session` cookie found.
    /// Returns `Err(ApiErrorr::BadAuthData) if session expired, fake or by other
    /// reason is not retrned by `UseSessionManager``
    pub async fn check_auth(&self, cookie: axum_extra::extract::CookieJar)
    -> Result<Option<i32>, ApiError> { 
        
        let session = match cookie.get("session") { 
            Some(s) => {
                let s = s.to_string();
                match uuid::Uuid::parse_str(&s) {
                    Ok(u) => u, 
                    Err(e) => {
                        debug!("bad session-uuid {e}");
                        return Err(ApiError::BadAuthData);
                    }
                }

            }, 
            None => {return Ok(None) }
        };        
        let usm = self.dbpool_session_manager.clone();
        match usm.resolve_user_session_to_id(session).await {
            Ok(i) => match i { Some(o) => return Ok(Some(o)), None => return Err(ApiError::BadAuthData) } ,
            Err(e) =>  {
                 warn!("Failed to check use-session, db error? {e}");
                 return  Err(ApiError::InternalError(e));
                }
        }

        
        todo!()
    }
}

/// Based on passed configs 
pub fn init_router(cfg: &crate::cfg::Config, state: AppState) -> axum::Router {
    let mut r: Router<AppState> = Router::new();
    r = r.route("/usrmngr/register", post(routes::admin_handlers::register));
    #[cfg(feature="apidoc")]   
    if cfg.api_cfg.host_doc { 
        r = r.nest("/doc",builded_openapi_for_router() );
        info!("OpenApi doc is included in routes!")
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
    let syaml = openapi.to_yaml().unwrap();
   // let swagger  utoipa_swagger_ui::SwaggerUi::
    

    router = router
       .route("/schema.yaml", get(|| async{syaml}));
    info!("ApiDoc included, see /doc/schema.yaml");

    

    #[cfg(feature="swagger")]
    {router = router.merge(utoipa_swagger_ui::SwaggerUi
        ::new("/swagger-ui").url("/doc/schema.yaml", openapi )
        .config(utoipa_swagger_ui::Config
            ::default().default_model_rendering("model")));
        info!("Swagger included at /doc/swagger-ui/")
    }
    router 

}
