use crate::{Sha256HashDTO, TimestampRfc3339, UuidDTO};
use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct GetArtifactInput {
    pub artifact_id: UuidDTO,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ArtifactDTO {
    pub id: UuidDTO,
    pub size: u64,
    pub hash: Sha256HashDTO,
    #[serde(with = "serde_bytes")]
    pub artifact: Vec<u8>,
    pub created_at: TimestampRfc3339,
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct GetArtifactResponse {
    pub artifact: ArtifactDTO,
}
