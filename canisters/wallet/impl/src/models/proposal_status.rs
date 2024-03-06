use candid::{CandidType, Deserialize};
use ic_canister_core::types::Timestamp;
use ic_canister_macros::{stable_object, storable};
use std::fmt::{Display, Formatter};

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

#[storable]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

impl Display for ProposalStatusType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ProposalStatusType::Created => write!(f, "created"),
            ProposalStatusType::Adopted => write!(f, "adopted"),
            ProposalStatusType::Rejected => write!(f, "rejected"),
            ProposalStatusType::Scheduled => write!(f, "scheduled"),
            ProposalStatusType::Processing => write!(f, "processing"),
            ProposalStatusType::Completed => write!(f, "completed"),
            ProposalStatusType::Failed => write!(f, "failed"),
            ProposalStatusType::Cancelled => write!(f, "cancelled"),
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
        assert_eq!(ProposalStatusType::Created.to_string(), "created");
        assert_eq!(ProposalStatusType::Adopted.to_string(), "adopted");
        assert_eq!(ProposalStatusType::Rejected.to_string(), "rejected");
        assert_eq!(ProposalStatusType::Scheduled.to_string(), "scheduled");
        assert_eq!(ProposalStatusType::Processing.to_string(), "processing");
        assert_eq!(ProposalStatusType::Completed.to_string(), "completed");
        assert_eq!(ProposalStatusType::Failed.to_string(), "failed");
        assert_eq!(ProposalStatusType::Cancelled.to_string(), "cancelled");
    }

    #[test]
    fn test_to_status_u8_representation() {
        assert_eq!(u8::from(ProposalStatusType::Created), 0);
        assert_eq!(u8::from(ProposalStatusType::Adopted), 1);
        assert_eq!(u8::from(ProposalStatusType::Rejected), 2);
        assert_eq!(u8::from(ProposalStatusType::Scheduled), 4);
        assert_eq!(u8::from(ProposalStatusType::Processing), 5);
        assert_eq!(u8::from(ProposalStatusType::Completed), 6);
        assert_eq!(u8::from(ProposalStatusType::Failed), 7);
        assert_eq!(u8::from(ProposalStatusType::Cancelled), 3);
    }

    #[test]
    fn test_from_status_u8_representation() {
        assert_eq!(
            ProposalStatusType::try_from(0),
            Ok(ProposalStatusType::Created)
        );
        assert_eq!(
            ProposalStatusType::try_from(1),
            Ok(ProposalStatusType::Adopted)
        );
        assert_eq!(
            ProposalStatusType::try_from(2),
            Ok(ProposalStatusType::Rejected)
        );
        assert_eq!(
            ProposalStatusType::try_from(4),
            Ok(ProposalStatusType::Scheduled)
        );
        assert_eq!(
            ProposalStatusType::try_from(5),
            Ok(ProposalStatusType::Processing)
        );
        assert_eq!(
            ProposalStatusType::try_from(6),
            Ok(ProposalStatusType::Completed)
        );
        assert_eq!(
            ProposalStatusType::try_from(7),
            Ok(ProposalStatusType::Failed)
        );
        assert_eq!(
            ProposalStatusType::try_from(3),
            Ok(ProposalStatusType::Cancelled)
        );
    }
}
