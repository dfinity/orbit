use ic_agent::AgentError;
use orbit_station_api::ApiErrorDTO;
use thiserror::Error;

pub type StationAgentResult<T> = Result<T, StationAgentError>;

// TODO: Wrap less stuff in the AgentError
#[derive(Error, Debug)]
pub enum StationAgentError {
    #[error("The station API returned an error: {:?}", 0)]
    ApiError(ApiErrorDTO),
    #[error("Failed to set up the agent: {0}")]
    AgentError(#[from] anyhow::Error),
    #[error("Failed to parse canid: {0}")]
    CandidParseError(#[from] candid::Error),
    #[error("Error in the IC agent: {0}")]
    IcAgentError(#[from] AgentError),
}

impl From<ApiErrorDTO> for StationAgentError {
    fn from(value: ApiErrorDTO) -> Self {
        Self::ApiError(value)
    }
}
