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
}

impl From<ApiErrorDTO> for StationAgentError {
    fn from(value: ApiErrorDTO) -> Self {
        Self::ApiError(value)
    }
}
