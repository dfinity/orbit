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
    Accepted = 0,
    Rejected = 1,
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
            0 => Ok(ProposalVoteStatus::Accepted),
            1 => Ok(ProposalVoteStatus::Rejected),
            _ => Err(()),
        }
    }
}

impl FromStr for ProposalVoteStatus {
    type Err = ();

    fn from_str(variant: &str) -> Result<ProposalVoteStatus, Self::Err> {
        match variant {
            "accepted" => Ok(ProposalVoteStatus::Accepted),
            "rejected" => Ok(ProposalVoteStatus::Rejected),
            _ => Err(()),
        }
    }
}

impl Display for ProposalVoteStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ProposalVoteStatus::Accepted => write!(f, "accepted"),
            ProposalVoteStatus::Rejected => write!(f, "rejected"),
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
        assert_eq!(ProposalVoteStatus::Accepted.to_string(), "accepted");
        assert_eq!(
            ProposalVoteStatus::from_str("accepted").unwrap(),
            ProposalVoteStatus::Accepted
        );
        assert_eq!(ProposalVoteStatus::Rejected.to_string(), "rejected");
        assert_eq!(
            ProposalVoteStatus::from_str("rejected").unwrap(),
            ProposalVoteStatus::Rejected
        );
    }

    #[test]
    fn test_status_number_representation() {
        assert_eq!(ProposalVoteStatus::Accepted as u8, 0);
        assert_eq!(
            ProposalVoteStatus::try_from(0).unwrap(),
            ProposalVoteStatus::Accepted
        );
        assert_eq!(ProposalVoteStatus::Rejected as u8, 1);
        assert_eq!(
            ProposalVoteStatus::try_from(1).unwrap(),
            ProposalVoteStatus::Rejected
        );
    }
}
