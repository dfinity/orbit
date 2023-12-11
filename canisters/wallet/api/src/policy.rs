use crate::UuidDTO;
use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum EvaluationStatusDTO {
    Adopted,
    Pending,
    Rejected,
}

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
    Auto(EvaluationStatusDTO),
    ApprovalThreshold(UserSpecifierDTO, f64),
    MinimumVotes(UserSpecifierDTO, u16),
    IsAddressKYC,
    Or(Vec<CriteriaDTO>),
    And(Vec<CriteriaDTO>),
    Not(Box<CriteriaDTO>),
}
