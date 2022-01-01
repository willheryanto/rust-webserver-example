use crate::infrastructure::adapters::user::postgres_adapter::PostgresAdapter;
use crate::modules::user::database::repository::UserRepository;
use crate::modules::user::commands::create_user::handler::CreateUserCommandHandler;
use crate::modules::user::commands::create_user::command::CreateUserCommand;
use crate::libs::ddd::domain::base_traits::command_handler_trait::CommandHandlerTrait;

use actix_web::{HttpResponse, web, post};
use serde::{Deserialize, Serialize};


pub struct CreateUserHttpController;

impl CreateUserHttpController {
    pub fn new() -> Self {
        Self {}
    }

    pub fn controller(&self) -> impl Fn(&mut web::ServiceConfig) {
        pub fn _controller(cfg: &mut web::ServiceConfig) {
            cfg.service(create_user_controller);
        }

        _controller
    }
}

#[derive(Deserialize, Serialize)]
struct CreateUserRequest {
    username: String,
    password: String,
}

#[post("/users")]
async fn create_user_controller(json: web::Json<CreateUserRequest> ) -> HttpResponse {
    let postgres_adapter = PostgresAdapter::new();
    let user_repository = UserRepository::new(&postgres_adapter);
    let create_user_command_handler = CreateUserCommandHandler::new(&user_repository);
    let create_user_command = CreateUserCommand::new(
        &json.0.username,
        &json.0.password,
    );

    let id = match create_user_command_handler.handle(&create_user_command).await {
        Ok(id) => id,
        Err(e) => {
            print!("{}", e);
            return HttpResponse::BadRequest().body("User already exists");
        }
    };

    HttpResponse::Ok().json(id)
}
