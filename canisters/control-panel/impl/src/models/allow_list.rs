use crate::errors::AllowListError;
use candid::Principal;
use ic_canister_core::model::{ModelValidator, ModelValidatorResult};
use ic_canister_macros::storable;

#[storable(serializer = "candid")]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct AllowListEntry {
    /// The principal of the user on the allow list.
    pub id: Principal,
    /// The e-mail address of the user.
    pub email: String,
    /// The existing wallets of the user.
    pub existing_wallets: Vec<Principal>,
    /// The removed wallets of the user.
    pub removed_wallets: Vec<Principal>,
    /// The main wallet of the user.
    pub main_wallet: Option<Principal>,
}

impl AllowListEntry {
    pub const MAX_EMAIL_LEN_RANGE: usize = 100;
    pub const MAX_WALLETS: usize = 5;
}

fn validate_allow_list_entry_email(entry: &AllowListEntry) -> ModelValidatorResult<AllowListError> {
    if entry.email.len() > AllowListEntry::MAX_EMAIL_LEN_RANGE {
        return Err(AllowListError::ValidationError {
            info: format!(
                "Too long e-mail address, expected length at most {} but got {}",
                AllowListEntry::MAX_EMAIL_LEN_RANGE,
                entry.email.len(),
            ),
        });
    }

    Ok(())
}

fn validate_allow_list_entry_wallets(
    entry: &AllowListEntry,
) -> ModelValidatorResult<AllowListError> {
    let number_wallets = entry.existing_wallets.len() + entry.removed_wallets.len();
    if number_wallets > AllowListEntry::MAX_WALLETS {
        return Err(AllowListError::ValidationError {
            info: format!(
                "Too many wallets, expected number at most {} but got {}",
                AllowListEntry::MAX_EMAIL_LEN_RANGE,
                number_wallets,
            ),
        });
    }

    for existing in entry.existing_wallets.iter() {
        if entry.removed_wallets.contains(existing) {
            return Err(AllowListError::ValidationError {
                info: format!(
                    "The existing wallet {} is listed among removed wallets.",
                    existing,
                ),
            });
        }
    }

    if let Some(main_wallet) = entry.main_wallet {
        if !entry.existing_wallets.contains(&main_wallet) {
            return Err(AllowListError::ValidationError {
                info: format!(
                    "Main wallet {} is not contained among existing wallets.",
                    main_wallet,
                ),
            });
        }
    }

    Ok(())
}

impl ModelValidator<AllowListError> for AllowListEntry {
    fn validate(&self) -> ModelValidatorResult<AllowListError> {
        validate_allow_list_entry_email(self)?;
        validate_allow_list_entry_wallets(self)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ic_stable_structures::Storable;

    #[test]
    fn valid_model_serialization() {
        let model = AllowListEntry {
            id: Principal::from_slice(&[u8::MAX; 29]),
            email: "john@example.com".to_string(),
            existing_wallets: vec![
                Principal::from_slice(&[0; 29]),
                Principal::from_slice(&[1; 29]),
            ],
            removed_wallets: vec![
                Principal::from_slice(&[2; 29]),
                Principal::from_slice(&[3; 29]),
            ],
            main_wallet: Some(Principal::from_slice(&[1; 29])),
        };

        let serialized_model = model.to_bytes();
        let deserialized_model = AllowListEntry::from_bytes(serialized_model);

        assert_eq!(model.id, deserialized_model.id);
        assert_eq!(model.email, deserialized_model.email);
        assert_eq!(model.existing_wallets, deserialized_model.existing_wallets);
        assert_eq!(model.removed_wallets, deserialized_model.removed_wallets);
        assert_eq!(model.main_wallet, deserialized_model.main_wallet);
    }

    #[test]
    fn valid_allow_list_entry() {
        let entry = AllowListEntry {
            id: Principal::from_slice(&[u8::MAX; 29]),
            email: format!(
                "{}@example.com",
                String::from_utf8(vec![b'j'; AllowListEntry::MAX_EMAIL_LEN_RANGE - 12]).unwrap()
            ),
            existing_wallets: (0..AllowListEntry::MAX_WALLETS - 1)
                .map(|i| Principal::from_slice(&[(i + 1) as u8; 29]))
                .collect(),
            removed_wallets: vec![Principal::from_slice(&[0; 29])],
            main_wallet: Some(Principal::from_slice(&[1; 29])),
        };

        assert!(validate_allow_list_entry_email(&entry).is_ok());
        assert!(validate_allow_list_entry_wallets(&entry).is_ok());
        assert!(entry.validate().is_ok());
    }

    #[test]
    fn invalid_allow_list_entry_email() {
        let entry = AllowListEntry {
            id: Principal::from_slice(&[u8::MAX; 29]),
            email: format!(
                "{}@example.com",
                String::from_utf8(vec![b'j'; AllowListEntry::MAX_EMAIL_LEN_RANGE - 12 + 1])
                    .unwrap()
            ),
            existing_wallets: (0..AllowListEntry::MAX_WALLETS - 1)
                .map(|i| Principal::from_slice(&[(i + 1) as u8; 29]))
                .collect(),
            removed_wallets: vec![Principal::from_slice(&[0; 29])],
            main_wallet: Some(Principal::from_slice(&[1; 29])),
        };

        assert!(validate_allow_list_entry_email(&entry).is_err());
        assert!(validate_allow_list_entry_wallets(&entry).is_ok());
        assert!(entry.validate().is_err());
    }

    #[test]
    fn too_many_allow_list_entry_wallets() {
        let entry = AllowListEntry {
            id: Principal::from_slice(&[u8::MAX; 29]),
            email: format!(
                "{}@example.com",
                String::from_utf8(vec![b'j'; AllowListEntry::MAX_EMAIL_LEN_RANGE - 12]).unwrap()
            ),
            existing_wallets: (0..AllowListEntry::MAX_WALLETS)
                .map(|i| Principal::from_slice(&[(i + 1) as u8; 29]))
                .collect(),
            removed_wallets: vec![Principal::from_slice(&[0; 29])],
            main_wallet: None,
        };

        assert!(validate_allow_list_entry_email(&entry).is_ok());
        assert!(validate_allow_list_entry_wallets(&entry).is_err());
        assert!(entry.validate().is_err());
    }

    #[test]
    fn allow_list_entry_wallets_disjointness() {
        let entry = AllowListEntry {
            id: Principal::from_slice(&[u8::MAX; 29]),
            email: format!(
                "{}@example.com",
                String::from_utf8(vec![b'j'; AllowListEntry::MAX_EMAIL_LEN_RANGE - 12]).unwrap()
            ),
            existing_wallets: vec![Principal::from_slice(&[0; 29])],
            removed_wallets: vec![Principal::from_slice(&[0; 29])],
            main_wallet: None,
        };

        assert!(validate_allow_list_entry_email(&entry).is_ok());
        assert!(validate_allow_list_entry_wallets(&entry).is_err());
        assert!(entry.validate().is_err());
    }

    #[test]
    fn invalid_allow_list_entry_main_wallet() {
        let entry = AllowListEntry {
            id: Principal::from_slice(&[u8::MAX; 29]),
            email: format!(
                "{}@example.com",
                String::from_utf8(vec![b'j'; AllowListEntry::MAX_EMAIL_LEN_RANGE - 12]).unwrap()
            ),
            existing_wallets: vec![Principal::from_slice(&[1; 29])],
            removed_wallets: vec![Principal::from_slice(&[0; 29])],
            main_wallet: Some(Principal::from_slice(&[0; 29])),
        };

        assert!(validate_allow_list_entry_email(&entry).is_ok());
        assert!(validate_allow_list_entry_wallets(&entry).is_err());
        assert!(entry.validate().is_err());
    }
}
