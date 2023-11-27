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
    EditAccount = 1,
    AddAccount = 2,
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
            1 => Ok(ProposalOperationType::EditAccount),
            2 => Ok(ProposalOperationType::AddAccount),
            _ => Err(()),
        }
    }
}

impl FromStr for ProposalOperationType {
    type Err = ();

    fn from_str(variant: &str) -> Result<ProposalOperationType, Self::Err> {
        match variant {
            "transfer" => Ok(ProposalOperationType::Transfer),
            "edit_account" => Ok(ProposalOperationType::EditAccount),
            "add_account" => Ok(ProposalOperationType::AddAccount),
            _ => Err(()),
        }
    }
}

impl Display for ProposalOperationType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ProposalOperationType::Transfer => write!(f, "transfer"),
            ProposalOperationType::EditAccount => write!(f, "edit_account"),
            ProposalOperationType::AddAccount => write!(f, "add_account"),
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
        assert_eq!(
            ProposalOperationType::EditAccount.to_string(),
            "edit_account"
        );
        assert_eq!(
            ProposalOperationType::from_str("edit_account").unwrap(),
            ProposalOperationType::EditAccount
        );
        assert_eq!(ProposalOperationType::AddAccount.to_string(), "add_account");
        assert_eq!(
            ProposalOperationType::from_str("add_account").unwrap(),
            ProposalOperationType::AddAccount
        );
    }

    #[test]
    fn operation_code_match_number_representation() {
        assert_eq!(ProposalOperationType::Transfer as u8, 0);
        assert_eq!(
            ProposalOperationType::try_from(0).unwrap(),
            ProposalOperationType::Transfer
        );
        assert_eq!(ProposalOperationType::EditAccount as u8, 1);
        assert_eq!(
            ProposalOperationType::try_from(1).unwrap(),
            ProposalOperationType::EditAccount
        );
        assert_eq!(ProposalOperationType::AddAccount as u8, 2);
        assert_eq!(
            ProposalOperationType::try_from(2).unwrap(),
            ProposalOperationType::AddAccount
        );
    }
}
