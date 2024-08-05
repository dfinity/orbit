use crate::models::ExternalCanisterId;
use orbit_essentials::storable;

/// The main index for external canisters.
#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExternalCanisterIndex {
    /// An indexed value of the external canister.
    pub index: ExternalCanisterIndexKind,
    /// The external canister id, which is a UUID.
    pub external_canister_id: ExternalCanisterId,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ExternalCanisterIndexKind {
    Name(String),
    Label(String),
}

#[derive(Clone, Debug)]
pub struct ExternalCanisterIndexCriteria {
    pub from: ExternalCanisterIndexKind,
    pub to: ExternalCanisterIndexKind,
}
