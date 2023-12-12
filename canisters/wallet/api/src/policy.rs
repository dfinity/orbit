use crate::UuidDTO;
use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum UserSpecifierDTO {
    Any,
    Group(Vec<UuidDTO>),
    Id(Vec<UuidDTO>),
    Owner,
    Proposer,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum CriteriaDTO {
    AutoAdopted,
    AutoRejected,
    ApprovalThreshold(UserSpecifierDTO, u16),
    MinimumVotes(UserSpecifierDTO, u16),
    Or(Vec<CriteriaDTO>),
    And(Vec<CriteriaDTO>),
    Not(Box<CriteriaDTO>),
}
