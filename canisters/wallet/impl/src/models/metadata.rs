use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Metadata {
    pub key: String,
    pub value: String,
}
