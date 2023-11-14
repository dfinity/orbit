use candid::{CandidType, Deserialize};
use ic_stable_structures::{storable::Bound, Storable};
use std::{
    borrow::Cow,
    fmt::{Display, Formatter},
    str::FromStr,
};

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum ProposalOperationType {
    Transfer = 0,
}

impl From<ProposalOperationType> for u8 {
    fn from(role: ProposalOperationType) -> Self {
        role as u8
    }
}

impl TryFrom<u8> for ProposalOperationType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ProposalOperationType::Transfer),
            _ => Err(()),
        }
    }
}

impl FromStr for ProposalOperationType {
    type Err = ();

    fn from_str(variant: &str) -> Result<ProposalOperationType, Self::Err> {
        match variant {
            "transfer" => Ok(ProposalOperationType::Transfer),
            _ => Err(()),
        }
    }
}

impl Display for ProposalOperationType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ProposalOperationType::Transfer => write!(f, "transfer"),
        }
    }
}

impl Storable for ProposalOperationType {
    fn to_bytes(&self) -> Cow<[u8]> {
        let operation_code_unit: u8 = self.to_owned().into();
        Cow::Owned(operation_code_unit.to_bytes().to_vec())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        let operation_code_unit = u8::from_bytes(bytes);
        ProposalOperationType::try_from(operation_code_unit).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operation_code_match_string_representation() {
        assert_eq!(ProposalOperationType::Transfer.to_string(), "transfer");
        assert_eq!(
            ProposalOperationType::from_str("transfer").unwrap(),
            ProposalOperationType::Transfer
        );
    }

    #[test]
    fn operation_code_match_number_representation() {
        assert_eq!(ProposalOperationType::Transfer as u8, 0);
        assert_eq!(
            ProposalOperationType::try_from(0).unwrap(),
            ProposalOperationType::Transfer
        );
    }
}
