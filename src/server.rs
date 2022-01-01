use crate::modules::healthcheck;
use crate::modules::movie::SearchMoviesHttpController;
use crate::modules::user::{CreateUserHttpController, FindUserHttpController};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    //let search_movies_controller = SearchMoviesController::new();
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .configure(healthcheck::init)
            //.configure(SearchMoviesController::controller)
            .configure(SearchMoviesHttpController::new().controller())
            .configure(CreateUserHttpController::new().controller())
            .configure(FindUserHttpController::new().controller())
            .app_data(web::Data::new(db_pool.clone()))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
