use crate::modules::movie::database::port::MovieRepositoryPort;
use crate::modules::movie::domain::entities::MovieEntity;
use std::io::Error;

pub struct MovieRepository<'a> {
    movie_repository: &'a dyn MovieRepositoryPort,
}

impl <'a> MovieRepository<'a> {
    pub fn new(movie_repository: &'a dyn MovieRepositoryPort) -> Self {
        Self { movie_repository }
    }

    pub async fn find_by_title(&self, title: String) -> Result<Vec<MovieEntity>, Error> {
        self.movie_repository.find_by_title(title).await
    }
}
