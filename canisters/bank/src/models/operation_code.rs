use candid::{CandidType, Deserialize};
use ic_stable_structures::{storable::Bound, Storable};
use std::{
    borrow::Cow,
    fmt::{Display, Formatter},
    str::FromStr,
};

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum OperationCode {
    ApproveTransfer = 0,
}

impl From<OperationCode> for u8 {
    fn from(role: OperationCode) -> Self {
        role as u8
    }
}

impl TryFrom<u8> for OperationCode {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(OperationCode::ApproveTransfer),
            _ => Err(()),
        }
    }
}

impl FromStr for OperationCode {
    type Err = ();

    fn from_str(variant: &str) -> Result<OperationCode, Self::Err> {
        match variant {
            "approve-transfer" => Ok(OperationCode::ApproveTransfer),
            _ => Err(()),
        }
    }
}

impl Display for OperationCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OperationCode::ApproveTransfer => write!(f, "approve-transfer"),
        }
    }
}

impl Storable for OperationCode {
    fn to_bytes(&self) -> Cow<[u8]> {
        let operation_code_unit: u8 = self.to_owned().into();
        Cow::Owned(operation_code_unit.to_bytes().to_vec())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        let operation_code_unit = u8::from_bytes(bytes);
        OperationCode::try_from(operation_code_unit).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}
