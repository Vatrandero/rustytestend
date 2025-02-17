use serde::{Deserialize, Serialize};
#[derive(Deserialize)]
pub struct User {
    pub id: u64,
    pub first_name: String,
    pub seocnd_name: Option<String>,
    pub last_name: Option<String>,
    pub login: String,
    pub password_hash: String,
}

impl User {
    pub fn new(
        id: u64,
        first_name: String,
        seocnd_name: Option<String>,
        last_name: Option<String>,
        login: String,
        password_hash: String,
    ) -> Self {
        Self {
            id,
            first_name,
            seocnd_name,
            last_name,
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
