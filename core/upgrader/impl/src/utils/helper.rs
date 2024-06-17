use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug)]
pub enum MapperError {
    MalformedUuid { malformed_uuid: String },
}

#[derive(Default, Clone, Debug)]
pub struct HelperMapper {}

impl HelperMapper {
    pub fn to_uuid(input_uuid: String) -> Result<Uuid, MapperError> {
        let uuid = Uuid::from_str(input_uuid.as_str()).map_err(|_| MapperError::MalformedUuid {
            malformed_uuid: input_uuid,
        })?;

        Ok(uuid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_uuid() {
        let uuid = Uuid::new_v4();
        let uuid_str = uuid.to_string();

        let result = HelperMapper::to_uuid(uuid_str.clone());

        assert_eq!(result.unwrap(), uuid);
    }
}
