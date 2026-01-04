use anyhow::Ok;
use argon2::{Argon2, PasswordHash, PasswordVerifier, password_hash::{PasswordHasher, SaltString, rand_core::OsRng}};
use serde::{Serialize, Deserialize};




#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
    iat: usize,
}

pub struct AuthService {
    jwt_secret: String,
    jwt_exp_seconds: i64,
}

impl AuthService {
    pub fn new(jwt_secret: String, jwt_exp_seconds: i64) -> Self {
        Self { jwt_secret, jwt_exp_seconds }
    }

    pub fn hash_password(&self, password: &str) -> anyhow::Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| anyhow::anyhow!("Failed to hash password: {}", e))?
            .to_string();
        Ok(password_hash)
    }

    pub fn verify_password(&self, hash: &str, password: &str) -> anyhow::Result<bool> {
        let parsed_hash = PasswordHash::new(hash)
                .map_err(|e| anyhow::anyhow!("Failed to parse hashed password {}", e))?;
        Ok(Ok(Argon2::default().verify_password(password.as_bytes(), &parsed_hash)).is_ok())
    }
   
}