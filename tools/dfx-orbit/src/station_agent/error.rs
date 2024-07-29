use ic_agent::AgentError;
use orbit_station_api::ApiErrorDTO;
use thiserror::Error;

pub type StationAgentResult<T> = Result<T, StationAgentError>;

#[derive(Error, Debug)]
pub enum StationAgentError {
    #[error("The station API returned an error: {:?}", 0)]
    Api(ApiErrorDTO),
    #[error("Failed to parse canid: {0}")]
    CandidParser(#[from] candid::Error),
    #[error("Error in the IC agent: {0}")]
    Agent(#[from] AgentError),
}

impl From<ApiErrorDTO> for StationAgentError {
    fn from(value: ApiErrorDTO) -> Self {
        Self::Api(value)
    }
}
