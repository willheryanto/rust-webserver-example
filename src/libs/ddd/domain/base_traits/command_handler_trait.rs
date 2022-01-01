use async_trait::async_trait;
use std::io::Result;
//use std::future::Future;

#[async_trait(?Send)]
pub trait CommandHandlerTrait<'a, C, R> {
    async fn handle(&self, query: &'a C) -> Result<R>;
    async fn execute(&self, query: &'a C) -> Result<R> {
        self.handle(query).await
    }
}

