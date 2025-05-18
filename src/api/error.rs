use axum::body::Body;
use axum::http::header::SET_COOKIE;
use axum::http::{Response, StatusCode};
use axum::response::IntoResponse;
use std::error::Error;

#[derive(Debug)]
pub enum ApiError {
    BadAuthData,                   // 401,403?
    UserError(Response<Body>),     //4xx, end-user-friednly error message. No masks.
    InternalError(Box<dyn Error>), // 500
    OtherErr(Box<dyn Error>),      // 500; we don't know what exactly happened, debug display..
}
impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::OtherErr(e) => write!(f, "{};; {:?}", self, e),
            _ => write!(f, "{}", self),
        }
    }
}

impl Error for ApiError {}

impl From<Box<dyn Error>> for ApiError {
    fn from(val: Box<dyn Error>) -> ApiError {
        Self::OtherErr(val)
    }
}
impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        match self {
            // We should not share internal error with end user.
            // log and mask
            Self::InternalError(e) => {
                warn!("Internal error detected: {}", e);
                return Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body("Internal server error.".into())
                    .unwrap();
            }
            // Whatever has user bad cookie or has error because
            // of it's noe existence, we need to clear `session` cookie
            Self::BadAuthData => {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .header(SET_COOKIE, "session=; Max-Age=0") // TODO: Make feature on conf deoended. end user may not want to unlogin even bad session coz main site may be unloged by misstake.
                    .body("Not authorized or bad autherecation data".into())
                    .unwrap();
            }
            Self::UserError(e) => e, // Just throw user error at user.
            Self::OtherErr(e) => {
                // Unknown error, mask as internal and log.
                error!("Unknown error, probaly internal error:  {}", e);
                return Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body("Internal server error.".into())
                    .unwrap();
            }
        }
    }
}
