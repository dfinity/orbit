use crate::errors::MetadataError;
use orbit_essentials::model::{ModelValidator, ModelValidatorResult};
use orbit_essentials::storable;
use station_api::MetadataDTO;
use std::collections::{BTreeMap, HashMap};

#[storable]
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Metadata {
    metadata: BTreeMap<String, String>,
}

#[storable]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MetadataItem {
    pub key: String,
    pub value: String,
}

#[storable]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ChangeMetadata {
    ReplaceAllBy(BTreeMap<String, String>),
    OverrideSpecifiedBy(BTreeMap<String, String>),
    RemoveKeys(Vec<String>),
}

impl Metadata {
    const MAX_METADATA: u8 = 10;
    const MAX_METADATA_KEY_LEN: u8 = 24;
    const MAX_METADATA_VALUE_LEN: u8 = 255;

    pub fn new(metadata: BTreeMap<String, String>) -> Self {
        Self { metadata }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.metadata.get(key).cloned()
    }

    pub fn keys(&self) -> Vec<&String> {
        self.metadata.keys().collect()
    }

    pub fn contains(&self, item: &MetadataItem) -> bool {
        self.metadata
            .get(&item.key)
            .map(|v| *v == item.value)
            .unwrap_or_default()
    }

    pub fn map(&self) -> HashMap<String, String> {
        self.metadata
            .iter()
            .map(|(k, v)| (k.to_owned(), v.to_owned()))
            .collect()
    }

    pub fn as_btreemap(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }

    pub(crate) fn change(&mut self, change_metadata: ChangeMetadata) {
        match change_metadata {
            ChangeMetadata::ReplaceAllBy(metadata) => {
                self.metadata = metadata;
            }
            ChangeMetadata::OverrideSpecifiedBy(metadata) => {
                for (key, value) in metadata {
                    self.metadata.insert(key, value);
                }
            }
            ChangeMetadata::RemoveKeys(keys) => {
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
