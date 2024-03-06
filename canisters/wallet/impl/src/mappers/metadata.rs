use crate::{
    errors::{AccountError, AddressBookError, MetadataError, TransferError},
    models::{ChangeMetadata, Metadata, MetadataItem},
};

impl From<Vec<wallet_api::MetadataDTO>> for Metadata {
    fn from(metadata_dto: Vec<wallet_api::MetadataDTO>) -> Self {
        let metadata = metadata_dto.into_iter().map(|m| (m.key, m.value)).collect();

        Self { metadata }
    }
}

impl From<Vec<MetadataItem>> for Metadata {
    fn from(metadata_item: Vec<MetadataItem>) -> Self {
        let metadata = metadata_item
            .into_iter()
            .map(|m| (m.key, m.value))
            .collect();

        Self { metadata }
    }
}

impl From<Metadata> for Vec<MetadataItem> {
    fn from(metadata: Metadata) -> Self {
        metadata
            .metadata
            .into_iter()
            .map(|(k, v)| MetadataItem { key: k, value: v })
            .collect()
    }
}

impl From<Metadata> for Vec<wallet_api::MetadataDTO> {
    fn from(metadata: Metadata) -> Self {
        metadata
            .metadata
            .into_iter()
            .map(|(k, v)| wallet_api::MetadataDTO { key: k, value: v })
            .collect()
    }
}

impl From<MetadataItem> for wallet_api::MetadataDTO {
    fn from(item: MetadataItem) -> Self {
        Self {
            key: item.key,
            value: item.value,
        }
    }
}

impl From<wallet_api::MetadataDTO> for MetadataItem {
    fn from(metadata_dto: wallet_api::MetadataDTO) -> Self {
        Self {
            key: metadata_dto.key,
            value: metadata_dto.value,
        }
    }
}

impl From<wallet_api::ChangeMetadataDTO> for ChangeMetadata {
    fn from(change_metadata_dto: wallet_api::ChangeMetadataDTO) -> Self {
        match change_metadata_dto {
            wallet_api::ChangeMetadataDTO::ReplaceAllBy(metadata_dto) => {
                let metadata = metadata_dto.into_iter().map(|m| (m.key, m.value)).collect();
                Self::ReplaceAllBy(metadata)
            }
            wallet_api::ChangeMetadataDTO::OverrideSpecifiedBy(metadata_dto) => {
                let metadata = metadata_dto.into_iter().map(|m| (m.key, m.value)).collect();
                Self::OverrideSpecifiedBy(metadata)
            }
            wallet_api::ChangeMetadataDTO::RemoveKeys(keys) => Self::RemoveKeys(keys),
        }
    }
}

impl From<ChangeMetadata> for wallet_api::ChangeMetadataDTO {
    fn from(change_metadata: ChangeMetadata) -> Self {
        match change_metadata {
            ChangeMetadata::ReplaceAllBy(metadata) => {
                let metadata_dto = metadata
                    .into_iter()
                    .map(|(k, v)| wallet_api::MetadataDTO { key: k, value: v })
                    .collect();
                Self::ReplaceAllBy(metadata_dto)
            }
            ChangeMetadata::OverrideSpecifiedBy(metadata) => {
                let metadata_dto = metadata
                    .into_iter()
                    .map(|(k, v)| wallet_api::MetadataDTO { key: k, value: v })
                    .collect();
                Self::OverrideSpecifiedBy(metadata_dto)
            }
            ChangeMetadata::RemoveKeys(keys) => Self::RemoveKeys(keys),
        }
    }
}

impl From<MetadataError> for AccountError {
    fn from(metadata_error: MetadataError) -> Self {
        match metadata_error {
            MetadataError::ValidationError { info: e } => Self::ValidationError { info: e },
        }
    }
}

impl From<MetadataError> for AddressBookError {
    fn from(metadata_error: MetadataError) -> Self {
        match metadata_error {
            MetadataError::ValidationError { info: e } => Self::ValidationError { info: e },
        }
    }
}

impl From<MetadataError> for TransferError {
    fn from(metadata_error: MetadataError) -> Self {
        match metadata_error {
            MetadataError::ValidationError { info: e } => Self::ValidationError { info: e },
        }
    }
}
