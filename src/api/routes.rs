pub(super) mod commons {
    pub use super::super::AppState;
    pub use crate::models;
    pub use axum::extract::{State, Query, Path};
    pub use axum::http::{header::SET_COOKIE, StatusCode};
    pub use axum::response::{AppendHeaders, IntoResponse};
    pub use axum::Json;
    pub use axum_extra::extract::CookieJar;
    pub use utoipa;
    pub use uuid::Uuid;
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
    #[utoipa::path(
            post,
            path="/usrmngr/register",
            responses( 
            (status = 200, description = "Registered"), 
            (status = 400, description ="Bad data provided or other reason"),
            (status = 500, description = "Internal failure, dtabase failure")
            ),
            request_body=models::dtos::UserRegisterReq)
            ]
    pub async  fn register(State(st): State<AppState>, 
    rq: Json<models::dtos::UserRegisterReq>) -> StatusCode {
        StatusCode::NOT_IMPLEMENTED
    }
}
pub mod user_handlers {
    use super::commons::*;
}
pub mod ktest_solver_handlers {
    use super::commons::*;
    #[utoipa::path(
        get,
        path = "/solver/test/list",
        responses(
            (status = 200, 
            description = "List test for requested solver. user_id never provided in body", 
            body = models::dtos::TestListResponse),
            (status = 401, description = "Not authorized"), 
            (status = 403, description = "Forbidden")
        ), 
        params(
            ("session" = Uuid, Cookie,
            description = "Session cookie, extracting user from it") 
        )
    )]    
    pub async fn list_test(
        State(st): State<AppState>, c: CookieJar,
        Query(rq): Query<i64>
    ) -> Result<Json<models::dtos::TestListResponse>, StatusCode> {
        Err(StatusCode::NOT_IMPLEMENTED)
    } 

    /// # Side effects:
    /// Begins test, creating record of it and
    /// providing user data of it.
    #[utoipa::path(
        post, 
        path = "/solver/test/{id}/start", 
        responses(
            (status=200, 
            description="Test succesufuly runed, returning",
            body = models::knowledge_test::KTestOngoing )
        ),
        params(
            ("id" = u64, Path, description = "Test nunber"),
            ("session" = Uuid, Cookie, 
            description = "Session cookie, extracting user from it")
        ) 
    )]
    pub async fn begin_ktest(State(st): State<AppState>, c: CookieJar,
     Query(rq): Query<i64>
) -> Result<Json<models::knowledge_test::KTestOngoing>, StatusCode> {
        Err(StatusCode::NOT_IMPLEMENTED)
    }
    
    /// Takes updateed KTestOngoin, verifies it and updates internally.
    #[utoipa::path(
        post, 
        path="/solver/ktsession/{id}/update",
        request_body=models::knowledge_test::KTestOngoing,
        responses(
            (status = 200, description = "Ok."),
            (status = 401, description = "Not autorized."), 
            (status = 422, description = "Bad data provided.")
        )
    )]
    pub async fn update_test_session(State(st): State<AppState>, c: CookieJar, 
    ) -> StatusCode {
        StatusCode::NOT_IMPLEMENTED
    }
    pub async fn handler(State(st): State<AppState>, c: CookieJar
    )   ->  impl IntoResponse {
        StatusCode::NOT_IMPLEMENTED
    }
}
pub mod ktest_manager_handlers {
    use crate::models::dtos::AsignToReq;

    use super::commons::*;

    #[utoipa::path(
        put, 
        path="/tmngr/add_test",
        request_body=models::dtos::KTestCreateReq,
        responses( 
            (status = 200, body=i64, description="id of created test"), 
            (status = 401, description= "Not authorized"), 
            (status = 422, description = "Bad test provided" ),
            (status = 500, description= "Internal server or database failure.")
        ), 
        params( 
            ("session" = Uuid, Cookie, 
            description = "User sessifon cookie, extracting user it from it")
        )
    )]
    pub async fn add_test(State(st): State<AppState>, c: CookieJar)
    ->  Result<i64, StatusCode> {
       Err(StatusCode::NOT_IMPLEMENTED)
    }

    #[utoipa::path(
        post, 
        path="/tmngr/{id}/delete_test",
        responses(
            (status = 200, description = "Ok" ),
            (status = 401, description = "Not authorized"),
            (status = 403, description = "Forbidden"), 
            (status = 500, description = "Inerrnal error or database error")
        ),
        params(
            ("session" = Uuid, Cookie, 
        description="User session, extracting user id from it."),
        ("id"=i64, Path, description="Test id to delete." )
        )
    )]
    pub async fn delete_test(State(st): State<AppState>, c: CookieJar)
        ->  impl IntoResponse {
        StatusCode::NOT_IMPLEMENTED
    }
    #[utoipa::path(
        post, 
        path="/tmngr/{id}/asign/",
        request_body=models::dtos::AsignToReq,
        responses(
            (status = 200, description = "OK, Asigned"), 
            (status = 422, description = "Bad data"),
            (status = 401, description = "Not authorized"),
            (status = 403, description = "Forbidden"),
            (status = 500, description = "Internal error or database error")
        ),
        params(
            ("session" = Uuid, Cookie, 
            description="User session, extracting user id from it." )
        )
    )]
    pub async fn asign_test(State(st): State<AppState>, c: CookieJar,
    rq: Json<AsignToReq>)
    ->  impl IntoResponse {
        StatusCode::NOT_IMPLEMENTED
    }
    #[utoipa::path(
        post, 
        path="/tmngr/{id}/unasign",
        request_body=models::dtos::UnAsignReq,
        responses(

        ),
        params(
            ("id"=i64, Path, description="Test ID."),
            ("sessino"=Uuid, Cookie, 
            description = "User session, extracting user id from it.")
        )
        
    )]
    pub async fn unasign_test(State(st): State<AppState>, c: CookieJar)
    ->  impl IntoResponse {
        StatusCode::NOT_IMPLEMENTED
    }

}
