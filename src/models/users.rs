use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type,  serde::Deserialize, serde::Serialize)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
    Admin,
    Solver, 
    //#[serde(rename="test_giver")]
    Test_Giver
}

#[derive(Deserialize, Serialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub second_name: Option<String>,
    pub last_name: Option<String>,
    pub user_role: UserRole,
    pub login: String,
    pub password_hash: String,
}

impl User {
    pub fn new(
        id: i32,
        first_name: String,
        second_name: Option<String>,
        last_name: Option<String>,
        login: String,
        user_role:UserRole,
        password_hash: String,
    ) -> Self {
        Self {
            id,
            first_name,
            second_name,
            last_name,
            user_role,
            login,
            password_hash,
        }
    }
}

/// Encapsulates password hashing and verification routines for user passwords.
/// Any changes to the hashing algorithm or configuration **must** remain internal to this module.
/// Exposes functions that always return easy-to-handle data types for outer scopes
pub mod passhash {
    use scrypt::{
        password_hash::{
            rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
        },
        Scrypt,
    };
    pub fn hash_password(password: &str) -> Result<String, Box<dyn std::error::Error>> {
        let salt = SaltString::generate(&mut OsRng);

        // Hash password to PHC string ($scrypt$...)
        // default params: 17,8,1.
        let password_hash = Scrypt
            .hash_password(password.as_bytes(), &salt)?
            .to_string();
        Ok(password_hash)
    }
    pub fn verify(password: &str, hash: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let parsed_hash = PasswordHash::new(hash)?;
        match Scrypt.verify_password(password.as_bytes(), &parsed_hash) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}
