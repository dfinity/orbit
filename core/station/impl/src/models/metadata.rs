use crate::{
    core::validation::{
        NumberFieldValidator, NumberFieldValidatorBuilder, StringFieldValidator,
        StringFieldValidatorBuilder, ValidateField,
    },
    errors::MetadataError,
};
use lazy_static::lazy_static;
use orbit_essentials::model::{ModelValidator, ModelValidatorResult};
use orbit_essentials::storable;
use station_api::MetadataDTO;
use std::collections::{BTreeMap, HashMap};

lazy_static! {
    pub static ref METADATA_COUNT_VALIDATOR: NumberFieldValidator<usize> = {
        NumberFieldValidatorBuilder::new("metadata_count".to_string())
            .max(Metadata::MAX_METADATA as usize)
            .build()
    };
    pub static ref METADATA_KEY_VALIDATOR: StringFieldValidator = {
        StringFieldValidatorBuilder::new("metadata_key".to_string())
            .max_length(Metadata::MAX_METADATA_KEY_LEN as usize)
            .build()
    };
    pub static ref METADATA_VALUE_VALIDATOR: StringFieldValidator = {
        StringFieldValidatorBuilder::new("metadata_value".to_string())
            .max_length(Metadata::MAX_METADATA_VALUE_LEN as usize)
            .build()
    };
}

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
    pub const MAX_METADATA_KEY_LEN: u8 = 24;
    pub const MAX_METADATA_VALUE_LEN: u8 = 255;

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
        METADATA_COUNT_VALIDATOR.validate_field(self.metadata.len())?;

        for (k, v) in self.metadata.iter() {
            METADATA_KEY_VALIDATOR.validate_field(k)?;
            METADATA_VALUE_VALIDATOR.validate_field(v)?;
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
                info: "The field `metadata_count` is invalid: Cannot be greater than 10."
                    .to_string(),
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
                info: "The field `metadata_key` is invalid: Length cannot be longer than 24."
                    .to_string(),
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
                info: "The field `metadata_value` is invalid: Length cannot be longer than 255."
                    .to_string(),
            }
        );
    }
}
