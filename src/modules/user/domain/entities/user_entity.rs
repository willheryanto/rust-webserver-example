use crate::modules::user::domain::value_objects::id::Id;
use crate::modules::user::domain::value_objects::password_hash::PasswordHash;
use secrecy::Secret;
use serde::{Deserialize};

#[derive(Deserialize)]
pub struct UserEntity {
    pub id: Id,
    pub username: String,
    pub password_hash: PasswordHash,
}

pub struct CreateUserProps {
    pub username: String,
    pub password: Secret<String>,
}

impl UserEntity {
    pub fn create(props: CreateUserProps) -> UserEntity {
        let id = Id::new();
        let password_hash = PasswordHash::new(props.password);

        UserEntity {
            id,
            username: props.username,
            password_hash,
        }
    }
}
