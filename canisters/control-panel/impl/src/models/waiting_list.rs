use crate::errors::WaitingListError;
use candid::Principal;
use ic_canister_core::model::{ModelValidator, ModelValidatorResult};
use ic_canister_macros::storable;

#[storable(serializer = "candid")]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WaitingListEntry {
    /// The principal of the user on the waiting list.
    pub id: Principal,
    /// The e-mail address of the user.
    pub email: String,
}

impl WaitingListEntry {
    pub const MAX_EMAIL_LEN_RANGE: usize = 100;
}

fn validate_waiting_list_entry(entry: &WaitingListEntry) -> ModelValidatorResult<WaitingListError> {
    if entry.email.len() > WaitingListEntry::MAX_EMAIL_LEN_RANGE {
        return Err(WaitingListError::ValidationError {
            info: format!(
                "Too long e-mail address, expected length at most {} but got {}",
                WaitingListEntry::MAX_EMAIL_LEN_RANGE,
                entry.email.len(),
            ),
        });
    }

    Ok(())
}

impl ModelValidator<WaitingListError> for WaitingListEntry {
    fn validate(&self) -> ModelValidatorResult<WaitingListError> {
        validate_waiting_list_entry(self)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ic_stable_structures::Storable;

    #[test]
    fn valid_model_serialization() {
        let model = WaitingListEntry {
            id: Principal::from_slice(&[u8::MAX; 29]),
            email: "john@example.com".to_string(),
        };

        let serialized_model = model.to_bytes();
        let deserialized_model = WaitingListEntry::from_bytes(serialized_model);

        assert_eq!(model.id, deserialized_model.id);
        assert_eq!(model.email, deserialized_model.email);
    }

    #[test]
    fn valid_waiting_list_entry() {
        let entry = WaitingListEntry {
            id: Principal::from_slice(&[u8::MAX; 29]),
            email: format!(
                "{}@example.com",
                String::from_utf8(vec![b'j'; WaitingListEntry::MAX_EMAIL_LEN_RANGE - 12]).unwrap()
            ),
        };

        assert!(validate_waiting_list_entry(&entry).is_ok());
        assert!(entry.validate().is_ok());
    }

    #[test]
    fn invalid_waiting_list_entry() {
        let entry = WaitingListEntry {
            id: Principal::from_slice(&[u8::MAX; 29]),
            email: format!(
                "{}@example.com",
                String::from_utf8(vec![b'j'; WaitingListEntry::MAX_EMAIL_LEN_RANGE - 12 + 1])
                    .unwrap()
            ),
        };

        assert!(validate_waiting_list_entry(&entry).is_err());
        assert!(entry.validate().is_err());
    }
}
