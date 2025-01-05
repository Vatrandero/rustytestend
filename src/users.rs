use serde::Deserialize;
#[derive(Deserialize)]
pub struct User{
    id: u64, 
    login: String, 
    password_hash: String,  // TODO: replace with Hash type when autherecation.    
                            // will be implemented.
    full_name: String
}
#[derive(Debug, Deserialize)]
pub struct UseAuthData { 
    login: String, 
    password: String  
}

