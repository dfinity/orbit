use crate::api::ApiError;

pub type ModelValidatorResult<Err = ApiError> = Result<(), Err>;

/// A repository is a generic interface for storing and retrieving data.
pub trait ModelValidator<Err = ApiError> {
    /// Returns the record from the repository if it exists.
    fn validate(&self) -> ModelValidatorResult<Err>;
}
