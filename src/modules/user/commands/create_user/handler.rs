use crate::modules::user::commands::create_user::command::CreateUserCommand;
use crate::modules::user::domain::entities::user_entity::{UserEntity, CreateUserProps};
use crate::modules::user::domain::value_objects::id::Id;
use crate::modules::user::database::repository::UserRepository;
use crate::libs::ddd::domain::base_traits::command_handler_trait::CommandHandlerTrait;

use async_trait::async_trait;
use secrecy::Secret;
//use std::io::{Error, ErrorKind};

pub struct CreateUserCommandHandler<'a> {
    user_repository: &'a UserRepository<'a>,
}

impl <'a> CreateUserCommandHandler<'a> {
    pub fn new(user_repository: &'a UserRepository) -> CreateUserCommandHandler<'a> {
        CreateUserCommandHandler { user_repository }
    }

}

#[async_trait(?Send)]
impl<'a> CommandHandlerTrait<'a, CreateUserCommand<'a>, Id> for CreateUserCommandHandler<'a> {
    async fn handle(&self, command: &'a CreateUserCommand<'a>) -> Result<Id, std::io::Error> {
        let user = UserEntity::create(CreateUserProps {
            username: command.username.into(),
            password: Secret::new(command.password.into())
        });

        self.user_repository.save(user).await
    }
}
