use super::dtos;

pub(super) mod commons {
    pub use axum::routing::{get, post, put};
    pub use axum::http::header::{SET_COOKIE};
    pub use axum::{Json, Router};
    pub use utoipa::openapi::OpenApi;
    
}

pub mod user_and_sesion_managment {
    use super::commons::*;
    /// NOTE: routes sttarts from root.
    /// NOTE: routes expected to correct nesting by api/mod.rs#init()
    pub fn get_router() -> Router {todo!()}
    pub fn get_openapi() -> OpenApi {todo!()}

        


}
pub mod admin {
    use axum::routing::{get, post, put};
    use axum::Json;
}
pub mod user_managment {
    use axum::routing::{get, post, put};
    use axum::Json;
}
pub mod ktest_managment {
    use axum::routing::{get, post, put};
    use axum::Json;
}