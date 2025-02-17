/// Contins structure used usuaaly by web api.
/// slices and or wrappers around other models.
/// Some used to generate obtain full object, some
/// passes thhrough  from api to place it needed without modification.


use serde::{Serialize, Deserialize};
pub use crate::models::knowledge_test::*;
pub use crate::models::users::*; 


#[derive(Debug, Deserialize)]
pub struct UserAuthReq {
    pub login: String,
    pub password: String,
}
#[derive(Deserialize)]
pub struct UserRegisterReq {
    pub login: String,
    pub password: String,
    pub first_name: String,
    pub seocnd_name: Option<String>,
    pub last_name: Option<String>,
    pub role_asigned: String, 
    pub groups_asigned: Option<Vec<String>>
}


pub struct TestListREsponse{ 
    
}