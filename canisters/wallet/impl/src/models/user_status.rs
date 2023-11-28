use candid::{CandidType, Deserialize};
use ic_stable_structures::{storable::Bound, Storable};
use std::{
    borrow::Cow,
    fmt::{Display, Formatter},
    str::FromStr,
};

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum UserStatus {
    Active = 0,
    Inactive = 1,
}

impl From<UserStatus> for u8 {
    fn from(status: UserStatus) -> Self {
        status as u8
    }
}

impl TryFrom<u8> for UserStatus {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(UserStatus::Active),
            1 => Ok(UserStatus::Inactive),
            _ => Err(()),
        }
    }
}

impl FromStr for UserStatus {
    type Err = ();

    fn from_str(variant: &str) -> Result<UserStatus, Self::Err> {
        match variant {
            "active" => Ok(UserStatus::Active),
            "inactive" => Ok(UserStatus::Inactive),
            _ => Err(()),
        }
    }
}

impl Display for UserStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UserStatus::Active => write!(f, "active"),
            UserStatus::Inactive => write!(f, "inactive"),
        }
    }
}

impl Storable for UserStatus {
    fn to_bytes(&self) -> Cow<[u8]> {
        let user_status: u8 = self.to_owned().into();
        Cow::Owned(user_status.to_bytes().to_vec())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        let user_status = u8::from_bytes(bytes);
        UserStatus::try_from(user_status).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_conversion() {
        assert_eq!(UserStatus::Active.to_string(), "active");
        assert_eq!(UserStatus::Inactive.to_string(), "inactive");
    }

    #[test]
    fn test_from_str() {
        assert_eq!(UserStatus::from_str("active").unwrap(), UserStatus::Active);
        assert_eq!(
            UserStatus::from_str("inactive").unwrap(),
            UserStatus::Inactive
        );
    }

    #[test]
    fn test_from_number() {
        assert_eq!(UserStatus::try_from(0).unwrap(), UserStatus::Active);
        assert_eq!(UserStatus::try_from(1).unwrap(), UserStatus::Inactive);
    }

    #[test]
    fn test_to_number() {
        assert_eq!(UserStatus::Active as u8, 0u8);
        assert_eq!(UserStatus::Inactive as u8, 1u8);
    }
}
