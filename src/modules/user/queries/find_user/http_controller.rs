use crate::infrastructure::adapters::user::postgres_adapter::PostgresAdapter;
use crate::modules::user::database::repository::UserRepository;
use crate::modules::user::queries::find_user::handler::FindUserQueryHandler;
use crate::modules::user::queries::find_user::query::FindUserQuery;
use crate::libs::ddd::domain::base_traits::query_handler_trait::QueryHandlerTrait;
use crate::authentication::{validate_credentials, Credentials};

use actix_web::{HttpResponse, web, post};
use serde::{Deserialize, Serialize};


pub struct FindUserHttpController;

impl FindUserHttpController {
    pub fn new() -> Self {
        Self {}
    }

    pub fn controller(&self) -> impl Fn(&mut web::ServiceConfig) {
        pub fn _controller(cfg: &mut web::ServiceConfig) {
            cfg.service(find_user_controller);
        }

        _controller
    }
}

#[derive(Deserialize, Serialize)]
struct FindUserRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct FindUserResponseData {
    id: String,
    username: String,
}

#[derive(Serialize)]
struct FindUserResponse {
    data: FindUserResponseData,
}

#[post("/sessions")]
async fn find_user_controller(json: web::Json<FindUserRequest> ) -> HttpResponse {
    let postgres_adapter = PostgresAdapter::new();
    let user_repository = UserRepository::new(&postgres_adapter);
    let find_user_query_handler = FindUserQueryHandler::new(&user_repository);

    let credentials = Credentials::new(
        json.0.username,
        json.0.password
    );
    let user_id = match validate_credentials(credentials, &postgres_adapter.pool).await {
        Ok(user_id) => user_id,
        Err(_) => {
            return HttpResponse::NotFound().json("User not found")
        }
    };

    let find_user_query = FindUserQuery::new(&user_id);

    match find_user_query_handler.handle(&find_user_query).await {
        Ok(user) => {
            return HttpResponse::Ok().json(FindUserResponse {
                data: FindUserResponseData {
                    id: user.id.into(),
                    username: user.username.into(),
                }
            })
        },
        Err(_) => {
            return HttpResponse::InternalServerError().json("Internal server error")
        }
    };
}
