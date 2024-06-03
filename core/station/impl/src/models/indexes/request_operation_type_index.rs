use crate::models::{
    request_operation_filter_type::RequestOperationFilterType, Request, RequestOperation,
};
use orbit_essentials::storable;
use orbit_essentials::types::UUID;

/// Index of requests by the approvers' user id.
#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RequestOperationTypeIndex {
    /// The operation type to filter by.
    pub operation_type: RequestOperationFilterType,
    /// The request id, which is a UUID.
    pub request_id: UUID,
}

#[derive(Clone, Debug)]
pub struct RequestOperationTypeIndexCriteria {
    pub operation_type: RequestOperationFilterType,
}

impl Request {
    pub fn to_index_by_operation_types(&self) -> Vec<RequestOperationTypeIndex> {
        match &self.operation {
            RequestOperation::Transfer(operation) => {
                vec![
                    RequestOperationTypeIndex {
                        operation_type: RequestOperationFilterType::Transfer(None),
                        request_id: self.id,
                    },
                    RequestOperationTypeIndex {
                        operation_type: RequestOperationFilterType::Transfer(Some(
                            operation.input.from_account_id,
                        )),
                        request_id: self.id,
                    },
                ]
            }
            RequestOperation::AddAccount(_) => vec![RequestOperationTypeIndex {
                operation_type: RequestOperationFilterType::AddAccount,
                request_id: self.id,
            }],
            RequestOperation::EditAccount(_) => vec![RequestOperationTypeIndex {
                operation_type: RequestOperationFilterType::EditAccount,
                request_id: self.id,
            }],
            RequestOperation::AddUser(_) => vec![RequestOperationTypeIndex {
                operation_type: RequestOperationFilterType::AddUser,
                request_id: self.id,
            }],
            RequestOperation::EditUser(_) => vec![RequestOperationTypeIndex {
                operation_type: RequestOperationFilterType::EditUser,
                request_id: self.id,
            }],
            RequestOperation::AddUserGroup(_) => vec![RequestOperationTypeIndex {
                operation_type: RequestOperationFilterType::AddUserGroup,
                request_id: self.id,
            }],
            RequestOperation::EditUserGroup(_) => vec![RequestOperationTypeIndex {
                operation_type: RequestOperationFilterType::EditUserGroup,
                request_id: self.id,
            }],
            RequestOperation::RemoveUserGroup(_) => vec![RequestOperationTypeIndex {
                operation_type: RequestOperationFilterType::RemoveUserGroup,
                request_id: self.id,
            }],
            RequestOperation::ChangeCanister(_) => vec![RequestOperationTypeIndex {
                operation_type: RequestOperationFilterType::ChangeCanister,
                request_id: self.id,
            }],
            RequestOperation::ChangeManagedCanister(operation) => vec![
                RequestOperationTypeIndex {
                    operation_type: RequestOperationFilterType::ChangeManagedCanister(None),
                    request_id: self.id,
                },
                RequestOperationTypeIndex {
                    operation_type: RequestOperationFilterType::ChangeManagedCanister(Some(
                        operation.input.canister_id,
                    )),
                    request_id: self.id,
                },
            ],
            RequestOperation::CreateManagedCanister(_) => vec![RequestOperationTypeIndex {
                operation_type: RequestOperationFilterType::CreateManagedCanister,
                request_id: self.id,
            }],
            RequestOperation::EditPermission(_) => vec![RequestOperationTypeIndex {
                operation_type: RequestOperationFilterType::EditPermission,
                request_id: self.id,
            }],
            RequestOperation::AddRequestPolicy(_) => vec![RequestOperationTypeIndex {
                operation_type: RequestOperationFilterType::AddRequestPolicy,
                request_id: self.id,
            }],
            RequestOperation::EditRequestPolicy(_) => vec![RequestOperationTypeIndex {
                operation_type: RequestOperationFilterType::EditRequestPolicy,
                request_id: self.id,
            }],
            RequestOperation::RemoveRequestPolicy(_) => vec![RequestOperationTypeIndex {
                operation_type: RequestOperationFilterType::RemoveRequestPolicy,
                request_id: self.id,
            }],
            RequestOperation::AddAddressBookEntry(_) => vec![RequestOperationTypeIndex {
                operation_type: RequestOperationFilterType::AddAddressBookEntry,
                request_id: self.id,
            }],
            RequestOperation::EditAddressBookEntry(_) => vec![RequestOperationTypeIndex {
                operation_type: RequestOperationFilterType::EditAddressBookEntry,
                request_id: self.id,
            }],
            RequestOperation::RemoveAddressBookEntry(_) => vec![RequestOperationTypeIndex {
                operation_type: RequestOperationFilterType::RemoveAddressBookEntry,
                request_id: self.id,
            }],
            RequestOperation::ManageSystemInfo(_) => vec![RequestOperationTypeIndex {
                operation_type: RequestOperationFilterType::ManageSystemInfo,
                request_id: self.id,
            }],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::models::{
        indexes::request_operation_type_index::RequestOperationTypeIndex,
        request_operation_filter_type::RequestOperationFilterType,
        request_test_utils::mock_request, Metadata, RequestOperation, TransferOperation,
        TransferOperationInput,
    };
    use ic_stable_structures::Storable;
    use num_bigint::BigUint;

    #[test]
    fn valid_model_serialization() {
        let request_id = [1; 16];
        let model = RequestOperationTypeIndex {
            operation_type: RequestOperationFilterType::RemoveAddressBookEntry,
            request_id,
        };

        let serialized_model = model.to_bytes();
        let deserialized_model = RequestOperationTypeIndex::from_bytes(serialized_model);

        assert_eq!(model.request_id, deserialized_model.request_id);
        assert_eq!(model.operation_type, deserialized_model.operation_type);
    }

    #[test]
    fn valid_user_approver_indexes() {
        let mut request = mock_request();
        let account_id = [0; 16];
        request.operation = RequestOperation::Transfer(TransferOperation {
            transfer_id: None,
            input: TransferOperationInput {
                amount: candid::Nat(BigUint::from(100u32)),
                fee: None,
                metadata: Metadata::default(),
                network: "mainnet".to_string(),
                to: "0x1234".to_string(),
                from_account_id: account_id,
            },
        });

        let indexes = request.to_index_by_operation_types();

        assert_eq!(indexes.len(), 2);
        assert!(indexes
            .iter()
            .any(|i| i.operation_type == RequestOperationFilterType::Transfer(None)));
        assert!(indexes
            .iter()
            .any(|i| i.operation_type == RequestOperationFilterType::Transfer(Some(account_id))));
    }
}
