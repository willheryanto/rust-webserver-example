use crate::modules::user::queries::find_user::query::FindUserQuery;
use crate::modules::user::domain::entities::UserEntity;
use crate::modules::user::database::repository::UserRepository;
use crate::libs::ddd::domain::base_traits::query_handler_trait::QueryHandlerTrait;

use async_trait::async_trait;

pub struct FindUserQueryHandler<'a> {
    user_repository: &'a UserRepository<'a>,
}

impl <'a> FindUserQueryHandler<'a> {
    pub fn new(user_repository: &'a UserRepository) -> FindUserQueryHandler<'a> {
        FindUserQueryHandler { user_repository }
    }

}

#[async_trait(?Send)]
impl<'a> QueryHandlerTrait<'a, FindUserQuery<'a>, UserEntity> for FindUserQueryHandler<'a> {
    async fn handle(&self, query: &'a FindUserQuery<'a>) -> Result<UserEntity, std::io::Error> {
        self.user_repository.find_one_by_id(query.id.clone()).await
    }
}
