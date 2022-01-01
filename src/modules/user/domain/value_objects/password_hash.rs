use crate::authentication::compute_password_hash;
use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PasswordHash(pub Secret<String>);

impl AsRef<str> for PasswordHash {
    fn as_ref(&self) -> &str {
        &self.0.expose_secret()
    }
}

impl PasswordHash {
    pub fn new(password: Secret<String>) -> PasswordHash {
        match compute_password_hash(password) {
            Ok(password_hash) => PasswordHash(password_hash),
            Err(_) => panic!("Error computing password hash."),
        }
    }
}
