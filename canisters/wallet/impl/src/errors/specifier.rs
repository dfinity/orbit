#[derive(Debug, thiserror::Error)]
pub enum MatchError {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}
