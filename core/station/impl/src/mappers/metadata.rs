use crate::{
    errors::{AccountError, AddressBookError, AssetError, MetadataError, TransferError},
    models::{ChangeMetadata, Metadata, MetadataItem},
};

impl From<Vec<station_api::MetadataDTO>> for Metadata {
    fn from(metadata_dto: Vec<station_api::MetadataDTO>) -> Self {
        let metadata = metadata_dto.into_iter().map(|m| (m.key, m.value)).collect();

        Metadata::new(metadata)
    }
}

impl From<Vec<MetadataItem>> for Metadata {
    fn from(metadata_item: Vec<MetadataItem>) -> Self {
        let metadata = metadata_item
            .into_iter()
            .map(|m| (m.key, m.value))
            .collect();

        Metadata::new(metadata)
    }
}

impl From<Metadata> for Vec<MetadataItem> {
    fn from(metadata: Metadata) -> Self {
        metadata
            .as_btreemap()
            .iter()
            .map(|(k, v)| MetadataItem {
                key: k.to_owned(),
                value: v.to_owned(),
            })
            .collect()
    }
}

impl From<Metadata> for Vec<station_api::MetadataDTO> {
    fn from(metadata: Metadata) -> Self {
        metadata
            .as_btreemap()
            .iter()
            .map(|(k, v)| station_api::MetadataDTO {
                key: k.to_owned(),
                value: v.to_owned(),
            })
            .collect()
    }
}

impl From<MetadataItem> for station_api::MetadataDTO {
    fn from(item: MetadataItem) -> Self {
        Self {
            key: item.key,
            value: item.value,
        }
    }
}

impl From<station_api::MetadataDTO> for MetadataItem {
    fn from(metadata_dto: station_api::MetadataDTO) -> Self {
        Self {
            key: metadata_dto.key,
            value: metadata_dto.value,
        }
    }
}

impl From<station_api::ChangeMetadataDTO> for ChangeMetadata {
    fn from(change_metadata_dto: station_api::ChangeMetadataDTO) -> Self {
        match change_metadata_dto {
            station_api::ChangeMetadataDTO::ReplaceAllBy(dto) => {
                let metadata = Metadata::from(dto);
                Self::ReplaceAllBy(metadata.as_btreemap().to_owned())
            }
            station_api::ChangeMetadataDTO::OverrideSpecifiedBy(dto) => {
                let metadata = Metadata::from(dto);
                Self::OverrideSpecifiedBy(metadata.as_btreemap().to_owned())
            }
            station_api::ChangeMetadataDTO::RemoveKeys(keys) => Self::RemoveKeys(keys),
        }
    }
}

impl From<ChangeMetadata> for station_api::ChangeMetadataDTO {
    fn from(change_metadata: ChangeMetadata) -> Self {
        match change_metadata {
            ChangeMetadata::ReplaceAllBy(metadata) => {
                let metadata = Metadata::new(metadata);
                Self::ReplaceAllBy(metadata.into())
            }
            ChangeMetadata::OverrideSpecifiedBy(metadata) => {
                let metadata = Metadata::new(metadata);
                Self::OverrideSpecifiedBy(metadata.into())
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

impl From<MetadataError> for AssetError {
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
