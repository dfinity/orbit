use orbit_essentials::storable;
use orbit_essentials::types::Timestamp;
use std::fmt::{Display, Formatter};

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RequestStatus {
    Created,
    Approved,
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
pub enum RequestStatusCode {
    Created = 0,
    Approved = 1,
    Rejected = 2,
    Cancelled = 3,
    Scheduled = 4,
    Processing = 5,
    Completed = 6,
    Failed = 7,
}

impl From<RequestStatus> for RequestStatusCode {
    fn from(status: RequestStatus) -> Self {
        match status {
            RequestStatus::Created => RequestStatusCode::Created,
            RequestStatus::Approved => RequestStatusCode::Approved,
            RequestStatus::Rejected => RequestStatusCode::Rejected,
            RequestStatus::Scheduled { .. } => RequestStatusCode::Scheduled,
            RequestStatus::Processing { .. } => RequestStatusCode::Processing,
            RequestStatus::Completed { .. } => RequestStatusCode::Completed,
            RequestStatus::Failed { .. } => RequestStatusCode::Failed,
            RequestStatus::Cancelled { .. } => RequestStatusCode::Cancelled,
        }
    }
}

impl From<RequestStatusCode> for u8 {
    fn from(status: RequestStatusCode) -> Self {
        status as u8
    }
}

impl TryFrom<u8> for RequestStatusCode {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(RequestStatusCode::Created),
            1 => Ok(RequestStatusCode::Approved),
            2 => Ok(RequestStatusCode::Rejected),
            3 => Ok(RequestStatusCode::Cancelled),
            4 => Ok(RequestStatusCode::Scheduled),
            5 => Ok(RequestStatusCode::Processing),
            6 => Ok(RequestStatusCode::Completed),
            7 => Ok(RequestStatusCode::Failed),
            _ => Err(()),
        }
    }
}

impl Display for RequestStatusCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RequestStatusCode::Created => write!(f, "created"),
            RequestStatusCode::Approved => write!(f, "approved"),
            RequestStatusCode::Rejected => write!(f, "rejected"),
            RequestStatusCode::Scheduled => write!(f, "scheduled"),
            RequestStatusCode::Processing => write!(f, "processing"),
            RequestStatusCode::Completed => write!(f, "completed"),
            RequestStatusCode::Failed => write!(f, "failed"),
            RequestStatusCode::Cancelled => write!(f, "cancelled"),
        }
    }
}

impl RequestStatus {
    pub fn to_type(&self) -> RequestStatusCode {
        match self {
            RequestStatus::Created => RequestStatusCode::Created,
            RequestStatus::Approved => RequestStatusCode::Approved,
            RequestStatus::Rejected => RequestStatusCode::Rejected,
            RequestStatus::Scheduled { .. } => RequestStatusCode::Scheduled,
            RequestStatus::Processing { .. } => RequestStatusCode::Processing,
            RequestStatus::Completed { .. } => RequestStatusCode::Completed,
            RequestStatus::Failed { .. } => RequestStatusCode::Failed,
            RequestStatus::Cancelled { .. } => RequestStatusCode::Cancelled,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_string_representation() {
        assert_eq!(RequestStatusCode::Created.to_string(), "created");
        assert_eq!(RequestStatusCode::Approved.to_string(), "approved");
        assert_eq!(RequestStatusCode::Rejected.to_string(), "rejected");
        assert_eq!(RequestStatusCode::Scheduled.to_string(), "scheduled");
        assert_eq!(RequestStatusCode::Processing.to_string(), "processing");
        assert_eq!(RequestStatusCode::Completed.to_string(), "completed");
        assert_eq!(RequestStatusCode::Failed.to_string(), "failed");
        assert_eq!(RequestStatusCode::Cancelled.to_string(), "cancelled");
    }

    #[test]
    fn test_to_status_u8_representation() {
        assert_eq!(u8::from(RequestStatusCode::Created), 0);
        assert_eq!(u8::from(RequestStatusCode::Approved), 1);
        assert_eq!(u8::from(RequestStatusCode::Rejected), 2);
        assert_eq!(u8::from(RequestStatusCode::Scheduled), 4);
        assert_eq!(u8::from(RequestStatusCode::Processing), 5);
        assert_eq!(u8::from(RequestStatusCode::Completed), 6);
        assert_eq!(u8::from(RequestStatusCode::Failed), 7);
        assert_eq!(u8::from(RequestStatusCode::Cancelled), 3);
    }

    #[test]
    fn test_from_status_u8_representation() {
        assert_eq!(
            RequestStatusCode::try_from(0),
            Ok(RequestStatusCode::Created)
        );
        assert_eq!(
            RequestStatusCode::try_from(1),
            Ok(RequestStatusCode::Approved)
        );
        assert_eq!(
            RequestStatusCode::try_from(2),
            Ok(RequestStatusCode::Rejected)
        );
        assert_eq!(
            RequestStatusCode::try_from(4),
            Ok(RequestStatusCode::Scheduled)
        );
        assert_eq!(
            RequestStatusCode::try_from(5),
            Ok(RequestStatusCode::Processing)
        );
        assert_eq!(
            RequestStatusCode::try_from(6),
            Ok(RequestStatusCode::Completed)
        );
        assert_eq!(
            RequestStatusCode::try_from(7),
            Ok(RequestStatusCode::Failed)
        );
        assert_eq!(
            RequestStatusCode::try_from(3),
            Ok(RequestStatusCode::Cancelled)
        );
    }
}
