use candid::{CandidType, Deserialize};
use ic_stable_structures::{storable::Bound, Storable};
use std::{
    borrow::Cow,
    fmt::{Display, Formatter},
    str::FromStr,
};

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum ProposalVoteStatus {
    Pending = 0,
    Adopted = 1,
    Rejected = 2,
    NotRequired = 3,
}

impl From<ProposalVoteStatus> for u8 {
    fn from(role: ProposalVoteStatus) -> Self {
        role as u8
    }
}

impl TryFrom<u8> for ProposalVoteStatus {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ProposalVoteStatus::Pending),
            1 => Ok(ProposalVoteStatus::Adopted),
            2 => Ok(ProposalVoteStatus::Rejected),
            3 => Ok(ProposalVoteStatus::NotRequired),
            _ => Err(()),
        }
    }
}

impl FromStr for ProposalVoteStatus {
    type Err = ();

    fn from_str(variant: &str) -> Result<ProposalVoteStatus, Self::Err> {
        match variant {
            "pending" => Ok(ProposalVoteStatus::Pending),
            "adopted" => Ok(ProposalVoteStatus::Adopted),
            "rejected" => Ok(ProposalVoteStatus::Rejected),
            "not-required" => Ok(ProposalVoteStatus::NotRequired),
            _ => Err(()),
        }
    }
}

impl Display for ProposalVoteStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ProposalVoteStatus::Pending => write!(f, "pending"),
            ProposalVoteStatus::Adopted => write!(f, "adopted"),
            ProposalVoteStatus::Rejected => write!(f, "rejected"),
            ProposalVoteStatus::NotRequired => write!(f, "not-required"),
        }
    }
}

impl Storable for ProposalVoteStatus {
    fn to_bytes(&self) -> Cow<[u8]> {
        let proposal_vote_status_unit: u8 = self.to_owned().into();
        Cow::Owned(proposal_vote_status_unit.to_bytes().to_vec())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        let proposal_vote_status_unit = u8::from_bytes(bytes);
        ProposalVoteStatus::try_from(proposal_vote_status_unit).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_string_representation() {
        assert_eq!(ProposalVoteStatus::Pending.to_string(), "pending");
        assert_eq!(
            ProposalVoteStatus::from_str("pending").unwrap(),
            ProposalVoteStatus::Pending
        );
        assert_eq!(ProposalVoteStatus::Adopted.to_string(), "adopted");
        assert_eq!(
            ProposalVoteStatus::from_str("adopted").unwrap(),
            ProposalVoteStatus::Adopted
        );
        assert_eq!(ProposalVoteStatus::Rejected.to_string(), "rejected");
        assert_eq!(
            ProposalVoteStatus::from_str("rejected").unwrap(),
            ProposalVoteStatus::Rejected
        );
        assert_eq!(ProposalVoteStatus::NotRequired.to_string(), "not-required");
        assert_eq!(
            ProposalVoteStatus::from_str("not-required").unwrap(),
            ProposalVoteStatus::NotRequired
        );
    }

    #[test]
    fn test_status_number_representation() {
        assert_eq!(ProposalVoteStatus::Pending as u8, 0);
        assert_eq!(
            ProposalVoteStatus::try_from(0).unwrap(),
            ProposalVoteStatus::Pending
        );
        assert_eq!(ProposalVoteStatus::Adopted as u8, 1);
        assert_eq!(
            ProposalVoteStatus::try_from(1).unwrap(),
            ProposalVoteStatus::Adopted
        );
        assert_eq!(ProposalVoteStatus::Rejected as u8, 2);
        assert_eq!(
            ProposalVoteStatus::try_from(2).unwrap(),
            ProposalVoteStatus::Rejected
        );
        assert_eq!(ProposalVoteStatus::NotRequired as u8, 3);
        assert_eq!(
            ProposalVoteStatus::try_from(3).unwrap(),
            ProposalVoteStatus::NotRequired
        );
    }
}
