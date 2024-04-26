use orbit_essentials::storable;
use orbit_essentials::types::Timestamp;
use std::fmt::{Display, Formatter};

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
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
pub enum ProposalStatusCode {
    Created = 0,
    Adopted = 1,
    Rejected = 2,
    Cancelled = 3,
    Scheduled = 4,
    Processing = 5,
    Completed = 6,
    Failed = 7,
}

impl From<ProposalStatusCode> for u8 {
    fn from(status: ProposalStatusCode) -> Self {
        status as u8
    }
}

impl TryFrom<u8> for ProposalStatusCode {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ProposalStatusCode::Created),
            1 => Ok(ProposalStatusCode::Adopted),
            2 => Ok(ProposalStatusCode::Rejected),
            3 => Ok(ProposalStatusCode::Cancelled),
            4 => Ok(ProposalStatusCode::Scheduled),
            5 => Ok(ProposalStatusCode::Processing),
            6 => Ok(ProposalStatusCode::Completed),
            7 => Ok(ProposalStatusCode::Failed),
            _ => Err(()),
        }
    }
}

impl Display for ProposalStatusCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ProposalStatusCode::Created => write!(f, "created"),
            ProposalStatusCode::Adopted => write!(f, "adopted"),
            ProposalStatusCode::Rejected => write!(f, "rejected"),
            ProposalStatusCode::Scheduled => write!(f, "scheduled"),
            ProposalStatusCode::Processing => write!(f, "processing"),
            ProposalStatusCode::Completed => write!(f, "completed"),
            ProposalStatusCode::Failed => write!(f, "failed"),
            ProposalStatusCode::Cancelled => write!(f, "cancelled"),
        }
    }
}

impl ProposalStatus {
    pub fn to_type(&self) -> ProposalStatusCode {
        match self {
            ProposalStatus::Created => ProposalStatusCode::Created,
            ProposalStatus::Adopted => ProposalStatusCode::Adopted,
            ProposalStatus::Rejected => ProposalStatusCode::Rejected,
            ProposalStatus::Scheduled { .. } => ProposalStatusCode::Scheduled,
            ProposalStatus::Processing { .. } => ProposalStatusCode::Processing,
            ProposalStatus::Completed { .. } => ProposalStatusCode::Completed,
            ProposalStatus::Failed { .. } => ProposalStatusCode::Failed,
            ProposalStatus::Cancelled { .. } => ProposalStatusCode::Cancelled,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_string_representation() {
        assert_eq!(ProposalStatusCode::Created.to_string(), "created");
        assert_eq!(ProposalStatusCode::Adopted.to_string(), "adopted");
        assert_eq!(ProposalStatusCode::Rejected.to_string(), "rejected");
        assert_eq!(ProposalStatusCode::Scheduled.to_string(), "scheduled");
        assert_eq!(ProposalStatusCode::Processing.to_string(), "processing");
        assert_eq!(ProposalStatusCode::Completed.to_string(), "completed");
        assert_eq!(ProposalStatusCode::Failed.to_string(), "failed");
        assert_eq!(ProposalStatusCode::Cancelled.to_string(), "cancelled");
    }

    #[test]
    fn test_to_status_u8_representation() {
        assert_eq!(u8::from(ProposalStatusCode::Created), 0);
        assert_eq!(u8::from(ProposalStatusCode::Adopted), 1);
        assert_eq!(u8::from(ProposalStatusCode::Rejected), 2);
        assert_eq!(u8::from(ProposalStatusCode::Scheduled), 4);
        assert_eq!(u8::from(ProposalStatusCode::Processing), 5);
        assert_eq!(u8::from(ProposalStatusCode::Completed), 6);
        assert_eq!(u8::from(ProposalStatusCode::Failed), 7);
        assert_eq!(u8::from(ProposalStatusCode::Cancelled), 3);
    }

    #[test]
    fn test_from_status_u8_representation() {
        assert_eq!(
            ProposalStatusCode::try_from(0),
            Ok(ProposalStatusCode::Created)
        );
        assert_eq!(
            ProposalStatusCode::try_from(1),
            Ok(ProposalStatusCode::Adopted)
        );
        assert_eq!(
            ProposalStatusCode::try_from(2),
            Ok(ProposalStatusCode::Rejected)
        );
        assert_eq!(
            ProposalStatusCode::try_from(4),
            Ok(ProposalStatusCode::Scheduled)
        );
        assert_eq!(
            ProposalStatusCode::try_from(5),
            Ok(ProposalStatusCode::Processing)
        );
        assert_eq!(
            ProposalStatusCode::try_from(6),
            Ok(ProposalStatusCode::Completed)
        );
        assert_eq!(
            ProposalStatusCode::try_from(7),
            Ok(ProposalStatusCode::Failed)
        );
        assert_eq!(
            ProposalStatusCode::try_from(3),
            Ok(ProposalStatusCode::Cancelled)
        );
    }
}
