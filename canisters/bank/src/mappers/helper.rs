use crate::errors::MapperError;
use num_bigint::BigUint;
use num_traits::ToPrimitive;
use std::str::FromStr;
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

    pub fn to_u64(input: &str) -> Result<u64, MapperError> {
        input
            .parse::<u64>()
            .map_err(|_| MapperError::StringToNumberConversionError {
                input: input.to_string(),
            })
    }

    pub fn biguint_to_u64(input: &BigUint) -> Result<u64, MapperError> {
        input.to_u64().ok_or(MapperError::BigUintConversionError {
            biguint: input.to_string(),
        })
    }
}
