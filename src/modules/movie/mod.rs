pub mod commands;
pub mod database;
pub mod domain;
pub mod queries;

pub use self::queries::search_movies::http_controller::SearchMoviesHttpController;
