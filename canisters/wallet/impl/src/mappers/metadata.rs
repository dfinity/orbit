use crate::errors::{AccountError, AddressBookError, MetadataError, TransferError};
use crate::models::Metadata;
use ic_canister_core::model::ModelValidatorResult;
use wallet_api::MetadataDTO;

impl From<Metadata> for MetadataDTO {
    fn from(metadata: Metadata) -> Self {
        Self {
            key: metadata.key,
            value: metadata.value,
        }
    }
}

impl From<MetadataDTO> for Metadata {
    fn from(metadata_dto: MetadataDTO) -> Self {
        Self {
            key: metadata_dto.key,
            value: metadata_dto.value,
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

impl Metadata {
    pub const MAX_METADATA: u8 = 10;
    pub const MAX_METADATA_KEY_LEN: u8 = 24;
    pub const MAX_METADATA_VALUE_LEN: u8 = 255;

    pub(crate) fn from_vec_dto(metadata: Vec<MetadataDTO>) -> Vec<Metadata> {
        metadata.into_iter().map(|m| m.into()).collect()
    }

    pub(crate) fn into_vec_dto(metadata: Vec<Metadata>) -> Vec<MetadataDTO> {
        metadata.into_iter().map(|m| m.into()).collect()
    }

    pub(crate) fn validate(metadata: &Vec<Metadata>) -> ModelValidatorResult<MetadataError> {
        if metadata.len() > Self::MAX_METADATA as usize {
            return Err(MetadataError::ValidationError {
                info: format!(
                    "Metadata count exceeds the maximum allowed: {}",
                    Self::MAX_METADATA
                ),
            });
        }

        for kv in metadata.iter() {
            if kv.key.len() > Self::MAX_METADATA_KEY_LEN as usize {
                return Err(MetadataError::ValidationError {
                    info: format!(
                        "Metadata key length exceeds the maximum allowed: {}",
                        Self::MAX_METADATA_KEY_LEN
                    ),
                });
            }

            if kv.value.len() > Self::MAX_METADATA_VALUE_LEN as usize {
                return Err(MetadataError::ValidationError {
                    info: format!(
                        "Metadata value length exceeds the maximum allowed: {}",
                        Self::MAX_METADATA_VALUE_LEN
                    ),
                });
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fail_metadata_validation_too_many() {
        let metadata = vec![
            Metadata {
                key: "a".repeat(Metadata::MAX_METADATA_KEY_LEN as usize),
                value: "b".repeat(Metadata::MAX_METADATA_VALUE_LEN as usize)
            };
            Metadata::MAX_METADATA as usize + 1
        ];

        let result = Metadata::validate(&metadata);

        assert_eq!(
            result.unwrap_err(),
            MetadataError::ValidationError {
                info: format!(
                    "Metadata count exceeds the maximum allowed: {}",
                    Metadata::MAX_METADATA
                ),
            }
        );
    }

    #[test]
    fn fail_metadata_validation_key_too_long() {
        let metadata = vec![
            Metadata {
                key: "a".repeat(Metadata::MAX_METADATA_KEY_LEN as usize + 1),
                value: "b".repeat(Metadata::MAX_METADATA_VALUE_LEN as usize)
            };
            Metadata::MAX_METADATA as usize
        ];

        let result = Metadata::validate(&metadata);

        assert_eq!(
            result.unwrap_err(),
            MetadataError::ValidationError {
                info: format!(
                    "Metadata key length exceeds the maximum allowed: {}",
                    Metadata::MAX_METADATA_KEY_LEN
                ),
            }
        );
    }

    #[test]
    fn fail_metadata_validation_value_too_long() {
        let metadata = vec![
            Metadata {
                key: "a".repeat(Metadata::MAX_METADATA_KEY_LEN as usize),
                value: "b".repeat(Metadata::MAX_METADATA_VALUE_LEN as usize + 1)
            };
            Metadata::MAX_METADATA as usize
        ];

        let result = Metadata::validate(&metadata);

        assert_eq!(
            result.unwrap_err(),
            MetadataError::ValidationError {
                info: format!(
                    "Metadata value length exceeds the maximum allowed: {}",
                    Metadata::MAX_METADATA_VALUE_LEN
                ),
            }
        );
    }
}
