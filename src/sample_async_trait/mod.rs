use async_trait::async_trait;

#[async_trait]
pub trait WithAsyncTrait {
    async fn f(&self) -> String;
}
