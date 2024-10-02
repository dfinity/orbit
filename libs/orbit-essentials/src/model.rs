use crate::{api::ApiError, types::UUID};

pub type ModelValidatorResult<Err = ApiError> = Result<(), Err>;

/// A repository is a generic interface for storing and retrieving data.
pub trait ModelValidator<Err = ApiError> {
    /// Returns the record from the repository if it exists.
    fn validate(&self) -> ModelValidatorResult<Err>;
}

#[derive(Debug, Clone)]
pub struct ContextualModel<M, C = ()> {
    pub model: M,
    pub context: C,
}

impl<M, C> ContextualModel<M, C> {
    pub fn new(model: M, context: C) -> Self {
        Self { model, context }
    }
}

/// A trait for models to expose their key.
pub trait ModelKey<Key = UUID> {
    fn key(&self) -> Key;
}
