use ic_agent::AgentError;
use orbit_station_api::ApiErrorDTO;
use thiserror::Error;

pub type StationAgentResult<T> = Result<T, StationAgentError>;

// TODO: Reduce things that are considered `OtherError`
#[derive(Error, Debug)]
pub enum StationAgentError {
    #[error("The station API returned an error: {:?}", 0)]
    ApiError(ApiErrorDTO),
    #[error("Failed to parse canid: {0}")]
    CandidParseError(#[from] candid::Error),
    #[error("Error in the IC agent: {0}")]
    AgentError(#[from] AgentError),
    #[error("Failed to make the request: {0}")]
    OtherError(#[from] anyhow::Error),
}

impl From<ApiErrorDTO> for StationAgentError {
    fn from(value: ApiErrorDTO) -> Self {
        Self::ApiError(value)
    }
}
