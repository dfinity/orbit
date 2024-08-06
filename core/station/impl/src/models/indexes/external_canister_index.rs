use crate::{
    core::utils::format_unique_string,
    models::{ExternalCanister, ExternalCanisterId},
};
use candid::Principal;
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
    CanisterId(Principal),
    Name(String),
    Label(String),
}

impl ExternalCanister {
    /// Converts the external canister to an index by its name.
    pub fn to_index_by_name(&self) -> ExternalCanisterIndex {
        ExternalCanisterIndex {
            index: ExternalCanisterIndexKind::Name(format_unique_string(self.name.as_str())),
            external_canister_id: self.id,
        }
    }

    /// Converts the external canister to indexes by its labels.
    pub fn to_index_by_labels(&self) -> Vec<ExternalCanisterIndex> {
        self.labels
            .iter()
            .map(|label| ExternalCanisterIndex {
                index: ExternalCanisterIndexKind::Label(format_unique_string(label.as_str())),
                external_canister_id: self.id,
            })
            .collect()
    }

    /// Converts the external canister to an index by its canister id.
    pub fn to_index_by_canister_id(&self) -> ExternalCanisterIndex {
        ExternalCanisterIndex {
            index: ExternalCanisterIndexKind::CanisterId(self.canister_id),
            external_canister_id: self.id,
        }
    }

    /// Converts the external canister to indexes to facilitate searching.
    pub fn indexes(&self) -> Vec<ExternalCanisterIndex> {
        let mut indexes = vec![self.to_index_by_name(), self.to_index_by_canister_id()];
        indexes.extend(self.to_index_by_labels());

        indexes
    }
}

#[derive(Clone, Debug)]
pub struct ExternalCanisterIndexCriteria {
    pub from: ExternalCanisterIndexKind,
    pub to: ExternalCanisterIndexKind,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::external_canister_test_utils::mock_external_canister;
    use ic_stable_structures::Storable;

    #[test]
    fn valid_model_serialization() {
        let model = ExternalCanisterIndex {
            index: ExternalCanisterIndexKind::CanisterId(Principal::anonymous()),
            external_canister_id: [u8::MAX; 16],
        };

        let serialized_model = model.to_bytes();
        let deserialized_model = ExternalCanisterIndex::from_bytes(serialized_model);

        assert_eq!(model.index, deserialized_model.index);
        assert_eq!(
            model.external_canister_id,
            deserialized_model.external_canister_id
        );
    }

    #[test]
    fn valid_external_canister_name_index_mapping() {
        let mut canister = mock_external_canister();
        canister.name = "Finance".to_string();

        let index = canister.to_index_by_name();

        assert_eq!(
            index.index,
            ExternalCanisterIndexKind::Name(format_unique_string("Finance"))
        );
        assert_eq!(index.external_canister_id, canister.id);
    }

    #[test]
    fn valid_external_canister_canister_id_index_mapping() {
        let mut canister = mock_external_canister();
        canister.canister_id = Principal::anonymous();

        let index = canister.to_index_by_canister_id();

        assert_eq!(
            index.index,
            ExternalCanisterIndexKind::CanisterId(Principal::anonymous())
        );
        assert_eq!(index.external_canister_id, canister.id);
    }

    #[test]
    fn valid_external_canister_label_index_mapping() {
        let mut canister = mock_external_canister();
        canister.labels = vec!["label-1".to_string(), "label-2".to_string()];

        let indexes = canister.to_index_by_labels();
        assert_eq!(indexes.len(), 2);

        let index = &indexes[0];
        assert_eq!(
            index.index,
            ExternalCanisterIndexKind::Label("label-1".to_string())
        );
        assert_eq!(index.external_canister_id, canister.id);

        let index = &indexes[1];
        assert_eq!(
            index.index,
            ExternalCanisterIndexKind::Label("label-2".to_string())
        );
        assert_eq!(index.external_canister_id, canister.id);
    }

    #[test]
    fn valid_external_canister_indexes_includes_all() {
        let mut canister = mock_external_canister();
        canister.name = "Finance".to_string();
        canister.canister_id = Principal::anonymous();
        canister.labels = vec!["label-1".to_string()];

        let indexes = canister.indexes();
        assert_eq!(indexes.len(), 3);
    }
}
