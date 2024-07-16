use dfx_core::error::{builder::BuildDfxInterfaceError, config::ConfigError};
use ic_agent::{export::PrincipalError, AgentError};
use orbit_station_api::ApiErrorDTO;
use thiserror::Error;

pub type StationAgentResult<T> = Result<T, StationAgentError>;

#[derive(Error, Debug)]
pub enum StationAgentError {
    #[error("Failed to parse principal: {0}")]
    PrincipalError(#[from] PrincipalError),
    #[error("The station API returned an error: {:?}", 0)]
    StationApiError(ApiErrorDTO),
    #[error("A call into dfx failed: arguments: {arguments}, err: {stderr}")]
    DfxCallError { arguments: String, stderr: String },
    #[error("Failed to construct interface to dfx: {0}")]
    BuildDfxInterfaceError(#[from] BuildDfxInterfaceError),
    #[error("Error in the dfx config: {0}")]
    DfxConfigError(#[from] ConfigError),
    #[error("The IC agent returned an error: {0}")]
    IcAgentError(#[from] AgentError),
    #[error("Error reading from disk: {0}")]
    IoError(#[from] std::io::Error),
    #[error("{0}")]
    Other(String),
}

impl From<ApiErrorDTO> for StationAgentError {
    fn from(value: ApiErrorDTO) -> Self {
        Self::StationApiError(value)
    }
}
