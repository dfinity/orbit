use crate::errors::EvaluateError;
use async_trait::async_trait;

#[async_trait]
pub trait Evaluate<T>: Send + Sync {
    async fn evaluate(&self) -> Result<T, EvaluateError>;
}
