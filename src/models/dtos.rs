
/// Contins structure used usuaaly by web api.
/// slices and or wrappers around other models.
/// Some used to generate obtain full object, some
/// passes thhrough  from api to place it needed without modification.
use utoipa::ToSchema;
pub use crate::models::knowledge_test::*;
pub use crate::models::users::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, ToSchema)]
pub struct UserAuthReq {
    pub login: String,
    pub password: String,
}
#[derive(Deserialize, ToSchema)]
pub struct UserRegisterReq {
    pub login: String,
    pub password: String,
    pub first_name: String,
    pub seocnd_name: Option<String>,
    pub last_name: Option<String>,
    pub role_asigned: String,
    pub groups_asigned: Option<Vec<String>>,
}
#[derive(Serialize, Deserialize, ToSchema)]
pub struct UserModReq{
    pub login: Option<String>,
    pub password: Option<String>,
    pub first_name: Option<String>,
    pub seocnd_name: Option<String>,
    pub last_name: Option<String>,
    pub role_asigned: Option<String>,
    pub groups_asigned: Option<Vec<String>>,
}

/*  Maybe this one should be used for admin only in admin forms?
Axim gives ability to return  Json<Vec<T>>, but when 
admin is requester - we may want to return 
something better formed.  */
#[derive(Serialize, ToSchema)]
pub struct TestListResponse {
    user_id: i64,
    asigned: Vec<AsignedTestResponse>

}
// builds from test and asignment.
#[derive(Serialize, Deserialize, ToSchema)]
pub struct AsignedTestResponse { 
    pub id: i64,  
    pub name: String,
    pub description: String, 
    pub duration: i64,
    pub attempts_avalable: i64,
    pub open_from: i64, 
    pub close_after: i64 
}
