use async_trait::async_trait;
use std::io::Result;
//use std::future::Future;

#[async_trait(?Send)]
pub trait QueryHandlerTrait<'a, Q, R> {
    async fn handle(&self, query: &'a Q) -> Result<R>;
    async fn execute(&self, query: &'a Q) -> Result<R> {
        self.handle(query).await
    }
}
