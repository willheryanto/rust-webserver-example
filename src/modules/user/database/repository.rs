use crate::modules::user::database::port::UserRepositoryPort;
use crate::modules::user::domain::entities::UserEntity;
use crate::modules::user::domain::value_objects::id::Id;

use uuid::Uuid;
use std::io::Error;

pub struct UserRepository<'a> {
    user_repository: &'a dyn UserRepositoryPort,
}

impl<'a> UserRepository<'a> {
    pub fn new(user_repository: &'a dyn UserRepositoryPort) -> Self {
        Self { user_repository }
    }

    pub async fn save(&self, user: UserEntity) -> Result<Id, Error> {
        match self.user_repository.save(user).await {
            Ok(id) => Ok(id),
            Err(err) => Err(err),
        }
    }

    pub async fn find_one_by_username(&self, username: String) -> Result<UserEntity, Error> {
        self.user_repository.find_one_by_username(username).await
    }

    pub async fn find_one_by_id(&self, id: Uuid) -> Result<UserEntity, Error> {
        self.user_repository.find_one_by_id(id).await
    }
}
