use candid::{CandidType, Deserialize};

#[derive(
    CandidType, serde::Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord,
)]
pub struct MetadataDTO {
    pub key: String,
    pub value: String,
}

#[derive(
    CandidType, serde::Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord,
)]
pub enum ChangeMetadataDTO {
    ReplaceAllBy(Vec<MetadataDTO>),
    OverrideSpecifiedBy(Vec<MetadataDTO>),
    RemoveKeys(Vec<String>),
}
