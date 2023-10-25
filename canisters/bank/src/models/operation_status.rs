use candid::{CandidType, Deserialize};
use ic_stable_structures::{storable::Bound, Storable};
use std::{
    borrow::Cow,
    fmt::{Display, Formatter},
    str::FromStr,
};

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum OperationStatus {
    Pending = 0,
    Adopted = 1,
    Rejected = 2,
    NotRequired = 3,
}

impl From<OperationStatus> for u8 {
    fn from(role: OperationStatus) -> Self {
        role as u8
    }
}

impl TryFrom<u8> for OperationStatus {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(OperationStatus::Pending),
            1 => Ok(OperationStatus::Adopted),
            2 => Ok(OperationStatus::Rejected),
            3 => Ok(OperationStatus::NotRequired),
            _ => Err(()),
        }
    }
}

impl FromStr for OperationStatus {
    type Err = ();

    fn from_str(variant: &str) -> Result<OperationStatus, Self::Err> {
        match variant {
            "pending" => Ok(OperationStatus::Pending),
            "adopted" => Ok(OperationStatus::Adopted),
            "rejected" => Ok(OperationStatus::Rejected),
            "not-required" => Ok(OperationStatus::NotRequired),
            _ => Err(()),
        }
    }
}

impl Display for OperationStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OperationStatus::Pending => write!(f, "pending"),
            OperationStatus::Adopted => write!(f, "adopted"),
            OperationStatus::Rejected => write!(f, "rejected"),
            OperationStatus::NotRequired => write!(f, "not-required"),
        }
    }
}

impl Storable for OperationStatus {
    fn to_bytes(&self) -> Cow<[u8]> {
        let operation_status_unit: u8 = self.to_owned().into();
        Cow::Owned(operation_status_unit.to_bytes().to_vec())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        let operation_status_unit = u8::from_bytes(bytes);
        OperationStatus::try_from(operation_status_unit).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}
