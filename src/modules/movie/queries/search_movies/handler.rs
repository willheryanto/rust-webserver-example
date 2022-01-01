use crate::modules::movie::queries::search_movies::query::SearchMoviesQuery;
use crate::modules::movie::domain::entities::MovieEntity;
use crate::modules::movie::database::repository::MovieRepository;
use crate::libs::ddd::domain::base_traits::query_handler_trait::QueryHandlerTrait;

use async_trait::async_trait;

pub struct SearchMoviesQueryHandler<'a> {
    movie_repository: &'a MovieRepository<'a>,
}

impl <'a> SearchMoviesQueryHandler<'a> {
    pub fn new(movie_repository: &'a MovieRepository) -> SearchMoviesQueryHandler<'a> {
        SearchMoviesQueryHandler { movie_repository }
    }

}

#[async_trait(?Send)]
impl<'a> QueryHandlerTrait<'a, SearchMoviesQuery<'a>, Vec<MovieEntity>> for SearchMoviesQueryHandler<'a> {
    async fn handle(&self, query: &'a SearchMoviesQuery<'a>) -> Result<Vec<MovieEntity>, std::io::Error> {
        self.movie_repository.find_by_title(query.title.into()).await
    }
}
