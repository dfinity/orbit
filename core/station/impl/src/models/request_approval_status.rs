use orbit_essentials::storable;
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum RequestApprovalStatus {
    Approved = 0,
    Rejected = 1,
}

impl From<RequestApprovalStatus> for u8 {
    fn from(status: RequestApprovalStatus) -> Self {
        status as u8
    }
}

impl TryFrom<u8> for RequestApprovalStatus {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(RequestApprovalStatus::Approved),
            1 => Ok(RequestApprovalStatus::Rejected),
            _ => Err(()),
        }
    }
}

impl FromStr for RequestApprovalStatus {
    type Err = ();

    fn from_str(variant: &str) -> Result<RequestApprovalStatus, Self::Err> {
        match variant {
            "approved" => Ok(RequestApprovalStatus::Approved),
            "rejected" => Ok(RequestApprovalStatus::Rejected),
            _ => Err(()),
        }
    }
}

impl Display for RequestApprovalStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RequestApprovalStatus::Approved => write!(f, "approved"),
            RequestApprovalStatus::Rejected => write!(f, "rejected"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_string_representation() {
        assert_eq!(RequestApprovalStatus::Approved.to_string(), "approved");
        assert_eq!(
            RequestApprovalStatus::from_str("approved").unwrap(),
            RequestApprovalStatus::Approved
        );
        assert_eq!(RequestApprovalStatus::Rejected.to_string(), "rejected");
        assert_eq!(
            RequestApprovalStatus::from_str("rejected").unwrap(),
            RequestApprovalStatus::Rejected
        );
    }

    #[test]
    fn test_status_number_representation() {
        assert_eq!(RequestApprovalStatus::Approved as u8, 0);
        assert_eq!(
            RequestApprovalStatus::try_from(0).unwrap(),
            RequestApprovalStatus::Approved
        );
        assert_eq!(RequestApprovalStatus::Rejected as u8, 1);
        assert_eq!(
            RequestApprovalStatus::try_from(1).unwrap(),
            RequestApprovalStatus::Rejected
        );
    }
}
