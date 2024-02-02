use crate::errors::{AccountError, AddressBookError, MetadataError, TransferError};
use candid::{CandidType, Deserialize};
use ic_canister_core::model::{ModelValidator, ModelValidatorResult};
use std::collections::{BTreeMap, HashMap};
use wallet_api::{ChangeMetadataDTO, MetadataDTO};

#[derive(CandidType, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Metadata {
    metadata: BTreeMap<String, String>,
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

impl From<Vec<MetadataDTO>> for Metadata {
    fn from(metadata_dto: Vec<MetadataDTO>) -> Self {
        let metadata = metadata_dto.into_iter().map(|m| (m.key, m.value)).collect();

        Self { metadata }
    }
}

impl Metadata {
    const MAX_METADATA: u8 = 10;
    const MAX_METADATA_KEY_LEN: u8 = 24;
    const MAX_METADATA_VALUE_LEN: u8 = 255;

    pub fn get(&self, key: &str) -> Option<String> {
        self.metadata.get(key).cloned()
    }

    pub fn keys(&self) -> Vec<&String> {
        self.metadata.keys().collect()
    }

    pub fn contains(&self, dto: MetadataDTO) -> bool {
        self.metadata
            .get(&dto.key)
            .map(|v| v.clone() == dto.value)
            .unwrap_or_default()
    }

    pub fn map(&self) -> HashMap<String, String> {
        self.metadata
            .iter()
            .map(|(k, v)| (k.to_owned(), v.to_owned()))
            .collect()
    }

    pub(crate) fn change(&mut self, change_metadata: ChangeMetadataDTO) {
        match change_metadata {
            ChangeMetadataDTO::ReplaceAllBy(metadata_dto) => {
                self.metadata = metadata_dto.into_iter().map(|m| (m.key, m.value)).collect();
            }
            ChangeMetadataDTO::OverrideSpecifiedBy(metadata_dto) => {
                for MetadataDTO { key, value } in metadata_dto {
                    self.metadata.insert(key, value);
                }
            }
            ChangeMetadataDTO::RemoveKeys(keys) => {
                for k in keys {
                    self.metadata.remove(&k);
                }
            }
        }
    }

    pub(crate) fn into_vec_dto(self) -> Vec<MetadataDTO> {
        self.metadata
            .into_iter()
            .map(|(k, v)| MetadataDTO { key: k, value: v })
            .collect()
    }

    #[cfg(test)]
    pub(crate) fn mock() -> Self {
        (0..Self::MAX_METADATA)
            .map(|i| MetadataDTO {
                key: format!("{:0>24}", i),
                value: "b".repeat(Self::MAX_METADATA_VALUE_LEN as usize),
            })
            .collect::<Vec<_>>()
            .into()
    }
}

impl ModelValidator<MetadataError> for Metadata {
    fn validate(&self) -> ModelValidatorResult<MetadataError> {
        if self.metadata.len() > Self::MAX_METADATA as usize {
            return Err(MetadataError::ValidationError {
                info: format!(
                    "Metadata count exceeds the maximum allowed: {}",
                    Self::MAX_METADATA
                ),
            });
        }

        for (k, v) in self.metadata.iter() {
            if k.len() > Self::MAX_METADATA_KEY_LEN as usize {
                return Err(MetadataError::ValidationError {
                    info: format!(
                        "Metadata key length exceeds the maximum allowed: {}",
                        Self::MAX_METADATA_KEY_LEN
                    ),
                });
            }

            if v.len() > Self::MAX_METADATA_VALUE_LEN as usize {
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
        let metadata: Metadata = (0..Metadata::MAX_METADATA as usize + 1)
            .map(|i| MetadataDTO {
                key: format!("{:0>24}", i),
                value: "b".repeat(Metadata::MAX_METADATA_VALUE_LEN as usize),
            })
            .collect::<Vec<_>>()
            .into();

        let result = metadata.validate();

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
        let metadata: Metadata = (0..Metadata::MAX_METADATA)
            .map(|i| MetadataDTO {
                key: format!("{:0>25}", i),
                value: "b".repeat(Metadata::MAX_METADATA_VALUE_LEN as usize),
            })
            .collect::<Vec<_>>()
            .into();

        let result = metadata.validate();

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
        let metadata: Metadata = (0..Metadata::MAX_METADATA)
            .map(|i| MetadataDTO {
                key: format!("{:0>24}", i),
                value: "b".repeat(Metadata::MAX_METADATA_VALUE_LEN as usize + 1),
            })
            .collect::<Vec<_>>()
            .into();

        let result = metadata.validate();

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
