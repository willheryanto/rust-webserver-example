use crate::modules::user::domain::entities::UserEntity;
use crate::modules::user::domain::value_objects::id::Id;

use async_trait::async_trait;
use uuid::Uuid;
use std::io::Error;

#[async_trait(?Send)]
pub trait UserRepositoryPort {
    async fn save(&self, entity: UserEntity) -> Result<Id, Error>;
    async fn find_one_by_username(&self, username: String) -> Result<UserEntity, Error>;
    async fn find_one_by_id(&self, id: Uuid) -> Result<UserEntity, Error>;
}
