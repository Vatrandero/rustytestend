pub(super) mod commons {
    pub use super::super::AppState;
    pub use crate::models;
    pub use axum::extract::{State, Query};
    pub use axum::http::{header::SET_COOKIE, StatusCode};
    pub use axum::response::{AppendHeaders, ErrorResponse, IntoResponse};
    pub use axum::routing::{get, post, put};
    pub use axum::{Json, Router};
    pub use axum_extra::extract::CookieJar;
    pub use utoipa;
    pub use utoipa::openapi::OpenApi;
}

pub mod user_and_sesion_managment {

    use super::commons::*;

    #[utoipa::path(
        post,
        path="/user/login", 
        request_body(content=models::dtos::UserAuthReq)

    )]
    pub async fn login(
        State(st): State<AppState>,
        rq: Json<models::dtos::UserAuthReq>,
    ) -> impl IntoResponse {
        StatusCode::NOT_IMPLEMENTED
    }
    // Asks db to delete session by id.
    pub async fn unlogin(State(st): State<AppState>, c: CookieJar) -> StatusCode {
        StatusCode::NOT_IMPLEMENTED
    }
}
pub mod admin_handlers {
    use super::commons::*;

    // Only admin shall register users.    // The first admin must be registered manually in DBMS
    // by a sysop or via the setup script.
    // TODO: consider adding a root user into migration scripts.
    #[utoipa::path(post,
            path="usermngr/register",
            request_body=models::dtos::UserRegisterReq)]
    pub fn register(State(st): State<AppState>, 
    rq: Json<models::dtos::UserRegisterReq>) -> StatusCode {
        StatusCode::NOT_IMPLEMENTED
    }
}
pub mod user_handlers {
    use super::commons::*;
}
pub mod ktest_solver_handlers {
    use super::commons::*;
    pub async fn list_test(
        State(st): State<AppState>, c: CookieJar,
        Query(rq): Query<Option<i64>> // Pass if asking for self.
    ) -> Result<Json<models::dtos::TestListResponse>, StatusCode> {
        Err(StatusCode::NOT_IMPLEMENTED)
    } 

    /// # Side effects:
    /// Begins test, creating record of it and
    /// providing user data of it.
    pub async fn begin_ktest(State(st): State<AppState>, c: CookieJar,
     Query(rq): Query<i64>
) -> Result<Json<models::knowledge_test::KTestOngoing>, StatusCode> {
        Err(StatusCode::NOT_IMPLEMENTED)
    }
    pub async fn answer_to_test(State(st): State<AppState>, c: CookieJar, 
    ) -> StatusCode {
        StatusCode::NOT_IMPLEMENTED
    }
    pub async fn handler(State(st): State<AppState>, c: CookieJar
    )   ->  impl IntoResponse {
        StatusCode::NOT_IMPLEMENTED
    }
}
pub mod ktest_manager_handlers {
    use super::commons::*;
}
