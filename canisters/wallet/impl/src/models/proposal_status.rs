use candid::{CandidType, Deserialize};
use ic_canister_core::types::Timestamp;
use ic_canister_macros::stable_object;
use ic_stable_structures::{storable::Bound, Storable};
use std::{
    borrow::Cow,
    fmt::{Display, Formatter},
};

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ProposalStatus {
    Created,
    Adopted,
    Rejected,
    Scheduled { scheduled_at: Timestamp },
    Cancelled { reason: Option<String> },
    Processing { started_at: Timestamp },
    Completed { completed_at: Timestamp },
    Failed { reason: Option<String> },
}

#[derive(CandidType, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum ProposalStatusType {
    Created = 0,
    Adopted = 1,
    Rejected = 2,
    Cancelled = 3,
    Scheduled = 4,
    Processing = 5,
    Completed = 6,
    Failed = 7,
}

impl From<ProposalStatusType> for u8 {
    fn from(status: ProposalStatusType) -> Self {
        status as u8
    }
}

impl TryFrom<u8> for ProposalStatusType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ProposalStatusType::Created),
            1 => Ok(ProposalStatusType::Adopted),
            2 => Ok(ProposalStatusType::Rejected),
            3 => Ok(ProposalStatusType::Cancelled),
            4 => Ok(ProposalStatusType::Scheduled),
            5 => Ok(ProposalStatusType::Processing),
            6 => Ok(ProposalStatusType::Completed),
            7 => Ok(ProposalStatusType::Failed),
            _ => Err(()),
        }
    }
}

impl Storable for ProposalStatusType {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned([self.to_owned() as u8].to_vec())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        let status = u8::from_bytes(bytes);
        ProposalStatusType::try_from(status).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

impl Display for ProposalStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ProposalStatus::Created => write!(f, "created"),
            ProposalStatus::Adopted => write!(f, "adopted"),
            ProposalStatus::Rejected => write!(f, "rejected"),
            ProposalStatus::Scheduled { .. } => write!(f, "scheduled"),
            ProposalStatus::Processing { .. } => write!(f, "processing"),
            ProposalStatus::Completed { .. } => write!(f, "completed"),
            ProposalStatus::Failed { .. } => write!(f, "failed"),
            ProposalStatus::Cancelled { .. } => write!(f, "cancelled"),
        }
    }
}

impl ProposalStatus {
    pub fn to_type(&self) -> ProposalStatusType {
        match self {
            ProposalStatus::Created => ProposalStatusType::Created,
            ProposalStatus::Adopted => ProposalStatusType::Adopted,
            ProposalStatus::Rejected => ProposalStatusType::Rejected,
            ProposalStatus::Scheduled { .. } => ProposalStatusType::Scheduled,
            ProposalStatus::Processing { .. } => ProposalStatusType::Processing,
            ProposalStatus::Completed { .. } => ProposalStatusType::Completed,
            ProposalStatus::Failed { .. } => ProposalStatusType::Failed,
            ProposalStatus::Cancelled { .. } => ProposalStatusType::Cancelled,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_string_representation() {
        assert_eq!(ProposalStatus::Created.to_string(), "created");
        assert_eq!(ProposalStatus::Adopted.to_string(), "adopted");
        assert_eq!(ProposalStatus::Rejected.to_string(), "rejected");
        assert_eq!(
            ProposalStatus::Scheduled { scheduled_at: 0 }.to_string(),
            "scheduled"
        );
        assert_eq!(
            ProposalStatus::Processing { started_at: 0 }.to_string(),
            "processing"
        );
        assert_eq!(
            ProposalStatus::Completed { completed_at: 0 }.to_string(),
            "completed"
        );
        assert_eq!(
            ProposalStatus::Failed { reason: None }.to_string(),
            "failed"
        );
        assert_eq!(
            ProposalStatus::Cancelled { reason: None }.to_string(),
            "cancelled"
        );
    }
}
