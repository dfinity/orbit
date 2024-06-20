use crate::core::ic_cdk::api::print;
use crate::errors::MapperError;
use std::{collections::BTreeMap, str::FromStr};
use uuid::Uuid;

#[derive(Default, Clone, Debug)]
pub struct HelperMapper {}

impl HelperMapper {
    pub fn to_uuid(input_uuid: String) -> Result<Uuid, MapperError> {
        let uuid = Uuid::from_str(input_uuid.as_str()).map_err(|_| MapperError::MalformedUuid {
            malformed_uuid: input_uuid,
        })?;

        Ok(uuid)
    }

    pub fn to_metadata(map: BTreeMap<String, String>) -> Vec<control_panel_api::MetadataDTO> {
        map.into_iter()
            .map(|(key, value)| control_panel_api::MetadataDTO { key, value })
            .collect()
    }

    pub fn from_metadata(
        metadata: Vec<control_panel_api::MetadataDTO>,
    ) -> BTreeMap<String, String> {
        metadata
            .into_iter()
            .map(|metadata| (metadata.key, metadata.value))
            .collect()
    }

    /// Converts a semver string to a semver version object.
    ///
    /// If the string is not a valid semver version, it will return 0.0.0 as a fallback.
    pub fn to_semver(version_text: &str) -> semver::Version {
        if let Ok(version) = semver::Version::parse(version_text) {
            return version;
        }

        print(format!(
            "Failed to parse semver {}, fallback to 0.0.0",
            version_text
        ));

        semver::Version::new(0, 0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_uuid_mapping() {
        let uuid = Uuid::new_v4();
        let uuid_string = uuid.to_string();

        let mapped_uuid = HelperMapper::to_uuid(uuid_string.clone()).unwrap();

        assert_eq!(uuid, mapped_uuid);
    }

    #[test]
    fn malformed_uuid_mapping() {
        let uuid_string = "malformed_uuid".to_string();
        let mapped_uuid = HelperMapper::to_uuid(uuid_string.clone());

        assert!(mapped_uuid.is_err());
    }
}
