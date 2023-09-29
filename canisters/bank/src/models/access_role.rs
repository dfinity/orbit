use candid::{CandidType, Deserialize};
use ic_stable_structures::{BoundedStorable, Storable};
use std::{
    borrow::Cow,
    fmt::{Display, Formatter},
    str::FromStr,
};

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum AccessRole {
    Admin = 1,
    User = 2,
}

impl From<AccessRole> for u8 {
    fn from(role: AccessRole) -> Self {
        role as u8
    }
}

impl TryFrom<u8> for AccessRole {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(AccessRole::Admin),
            2 => Ok(AccessRole::User),
            _ => Err(()),
        }
    }
}

impl FromStr for AccessRole {
    type Err = ();

    fn from_str(variant: &str) -> Result<AccessRole, Self::Err> {
        match variant {
            "admin" => Ok(AccessRole::Admin),
            "user" => Ok(AccessRole::User),
            _ => Err(()),
        }
    }
}

impl Display for AccessRole {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AccessRole::Admin => write!(f, "admin"),
            AccessRole::User => write!(f, "user"),
        }
    }
}

impl Storable for AccessRole {
    fn to_bytes(&self) -> Cow<[u8]> {
        let access_role_unit: u8 = self.to_owned().into();
        Cow::Owned(access_role_unit.to_bytes().to_vec())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        let access_role_unit = u8::from_bytes(bytes);
        AccessRole::try_from(access_role_unit).unwrap()
    }
}

impl BoundedStorable for AccessRole {
    const MAX_SIZE: u32 = 2;

    const IS_FIXED_SIZE: bool = false;
}
