use crate::modules::movie::domain::entities::MovieEntity;

use async_trait::async_trait;

#[async_trait]
pub trait MovieRepositoryPort {
    async fn find_by_title(&self, title: String) -> Result<Vec<MovieEntity>, std::io::Error>;
}
