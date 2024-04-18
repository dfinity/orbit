use crate::errors::MapperError;
use candid::Nat;
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

    pub fn nat_to_u64(amount: Nat) -> Result<u64, MapperError> {
        let amount_str = amount.to_string();

        amount
            .0
            .try_into()
            .map_err(|_| MapperError::NatConversionError { nat: amount_str })
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

    #[test]
    fn test_to_u64() {
        let input = "100";

        let result = HelperMapper::to_u64(input);

        assert_eq!(result.unwrap(), 100);
    }

    #[test]
    fn test_nat_to_u64() {
        let amount = Nat::from(100u32);

        let result = HelperMapper::nat_to_u64(amount);

        assert_eq!(result.unwrap(), 100);
    }

    #[test]
    fn test_to_uuid_should_fail() {
        let malformed_uuid = "malformed_uuid";

        let result = HelperMapper::to_uuid(malformed_uuid.to_string());

        assert!(result.is_err());
    }

    #[test]
    fn test_to_u64_should_fail() {
        let input = "not_a_number";

        let result = HelperMapper::to_u64(input);

        assert!(result.is_err());
    }

    #[test]
    fn test_nat_max_u64_should_convert() {
        let amount = Nat::from(u64::MAX);

        let result = HelperMapper::nat_to_u64(amount);

        assert_eq!(result.unwrap(), u64::MAX);
    }
}
