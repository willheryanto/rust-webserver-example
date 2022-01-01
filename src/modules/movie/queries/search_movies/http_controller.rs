use crate::modules::movie::queries::search_movies::query::SearchMoviesQuery;
use crate::modules::movie::queries::search_movies::handler::SearchMoviesQueryHandler;
use crate::modules::movie::database::repository::MovieRepository;
use crate::modules::movie::domain::entities::MovieEntity;
use crate::infrastructure::adapters::movie::moviedb_adapter::MoviedbAdapter;
use crate::libs::ddd::domain::base_traits::query_handler_trait::QueryHandlerTrait;

use actix_web::{get, web, HttpResponse};
use serde::{Serialize, Deserialize};

pub struct SearchMoviesHttpController;

impl SearchMoviesHttpController {
    pub fn new() -> Self {
        Self {}
    }

    pub fn controller(&self) -> impl Fn(&mut web::ServiceConfig) {
        pub fn _controller(cfg: &mut web::ServiceConfig) {
            cfg.service(search_movies_controller);
        }

        _controller
    }
}

#[derive(Deserialize, Debug)]
struct MovieSearch {
    movie_title: String
}

#[derive(Serialize)]
struct MovieResponse {
    movies: Vec<MovieEntity>
}

#[get("/movies")]
async fn search_movies_controller(query: web::Query<MovieSearch>) -> HttpResponse{
    let movie_db_adapter = MoviedbAdapter::new();
    let movie_repository = MovieRepository::new(&movie_db_adapter);
    let search_movie_query_handler = SearchMoviesQueryHandler::new(&movie_repository);

    let search_movie_query = SearchMoviesQuery {
        title: &query.0.movie_title
    };
    let movies = search_movie_query_handler.handle(&search_movie_query).await.expect("failed to search movie");

    HttpResponse::Ok().json(MovieResponse {
        movies
    })
}
