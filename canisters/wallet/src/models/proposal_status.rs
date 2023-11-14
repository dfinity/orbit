use candid::{CandidType, Deserialize};
use ic_stable_structures::{storable::Bound, Storable};
use std::{
    borrow::Cow,
    fmt::{Display, Formatter},
    str::FromStr,
};

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum ProposalStatus {
    Pending = 0,
    Adopted = 1,
    Rejected = 2,
}

impl From<ProposalStatus> for u8 {
    fn from(role: ProposalStatus) -> Self {
        role as u8
    }
}

impl TryFrom<u8> for ProposalStatus {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ProposalStatus::Pending),
            1 => Ok(ProposalStatus::Adopted),
            2 => Ok(ProposalStatus::Rejected),
            _ => Err(()),
        }
    }
}

impl FromStr for ProposalStatus {
    type Err = ();

    fn from_str(variant: &str) -> Result<ProposalStatus, Self::Err> {
        match variant {
            "pending" => Ok(ProposalStatus::Pending),
            "adopted" => Ok(ProposalStatus::Adopted),
            "rejected" => Ok(ProposalStatus::Rejected),
            _ => Err(()),
        }
    }
}

impl Display for ProposalStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ProposalStatus::Pending => write!(f, "pending"),
            ProposalStatus::Adopted => write!(f, "adopted"),
            ProposalStatus::Rejected => write!(f, "rejected"),
        }
    }
}

impl Storable for ProposalStatus {
    fn to_bytes(&self) -> Cow<[u8]> {
        let proposal_status_unit: u8 = self.to_owned().into();
        Cow::Owned(proposal_status_unit.to_bytes().to_vec())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        let proposal_status_unit = u8::from_bytes(bytes);
        ProposalStatus::try_from(proposal_status_unit).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_string_representation() {
        assert_eq!(ProposalStatus::Pending.to_string(), "pending");
        assert_eq!(
            ProposalStatus::from_str("pending").unwrap(),
            ProposalStatus::Pending
        );
        assert_eq!(ProposalStatus::Adopted.to_string(), "adopted");
        assert_eq!(
            ProposalStatus::from_str("adopted").unwrap(),
            ProposalStatus::Adopted
        );
        assert_eq!(ProposalStatus::Rejected.to_string(), "rejected");
        assert_eq!(
            ProposalStatus::from_str("rejected").unwrap(),
            ProposalStatus::Rejected
        );
    }

    #[test]
    fn test_status_number_representation() {
        assert_eq!(ProposalStatus::Pending as u8, 0);
        assert_eq!(
            ProposalStatus::try_from(0).unwrap(),
            ProposalStatus::Pending
        );
        assert_eq!(ProposalStatus::Adopted as u8, 1);
        assert_eq!(
            ProposalStatus::try_from(1).unwrap(),
            ProposalStatus::Adopted
        );
        assert_eq!(ProposalStatus::Rejected as u8, 2);
        assert_eq!(
            ProposalStatus::try_from(2).unwrap(),
            ProposalStatus::Rejected
        );
    }
}
