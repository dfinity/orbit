use candid::{CandidType, Deserialize};
use ic_stable_structures::{storable::Bound, Storable};
use std::{
    borrow::Cow,
    fmt::{Display, Formatter},
    str::FromStr,
};

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum NotificationStatus {
    Sent = 0,
    Read = 1,
}

impl From<NotificationStatus> for u8 {
    fn from(status: NotificationStatus) -> Self {
        status as u8
    }
}

impl TryFrom<u8> for NotificationStatus {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(NotificationStatus::Sent),
            1 => Ok(NotificationStatus::Read),
            _ => Err(()),
        }
    }
}

impl FromStr for NotificationStatus {
    type Err = ();

    fn from_str(variant: &str) -> Result<NotificationStatus, Self::Err> {
        match variant {
            "sent" => Ok(NotificationStatus::Sent),
            "read" => Ok(NotificationStatus::Read),
            _ => Err(()),
        }
    }
}

impl Display for NotificationStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NotificationStatus::Sent => write!(f, "sent"),
            NotificationStatus::Read => write!(f, "read"),
        }
    }
}

impl Storable for NotificationStatus {
    fn to_bytes(&self) -> Cow<[u8]> {
        let status_unit: u8 = self.to_owned().into();
        Cow::Owned(status_unit.to_bytes().to_vec())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        let status_unit = u8::from_bytes(bytes);
        NotificationStatus::try_from(status_unit).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_string_representation() {
        assert_eq!(NotificationStatus::Sent.to_string(), "sent");
        assert_eq!(
            NotificationStatus::from_str("sent").unwrap(),
            NotificationStatus::Sent
        );
        assert_eq!(NotificationStatus::Read.to_string(), "read");
        assert_eq!(
            NotificationStatus::from_str("read").unwrap(),
            NotificationStatus::Read
        );
    }

    #[test]
    fn test_status_number_representation() {
        assert_eq!(NotificationStatus::Sent as u8, 0);
        assert_eq!(
            NotificationStatus::try_from(0).unwrap(),
            NotificationStatus::Sent
        );
        assert_eq!(NotificationStatus::Read as u8, 1);
        assert_eq!(
            NotificationStatus::try_from(1).unwrap(),
            NotificationStatus::Read
        );
    }
}
