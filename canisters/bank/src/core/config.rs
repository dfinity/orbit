use super::WASM_PAGE_SIZE;
use crate::{
    core::ic_cdk::api::{time, trap},
    models::{AccessRole, BankAsset, Blockchain, BlockchainStandard, WalletPolicy},
    transport::{BankPermissionDTO, UserRoleDTO},
};
use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use ic_canister_core::types::Timestamp;
use ic_canister_macros::stable_object;
use ic_stable_structures::{storable::Bound, Storable};
use std::{
    borrow::Cow,
    cell::RefCell,
    collections::{HashMap, HashSet},
};

/// The list of permissions that can be granted to roles, admin role has all permissions.
pub const PERMISSION_ADMIN: &str = "admin";
pub const PERMISSION_READ_FEATURES: &str = "read:features";
pub const PERMISSION_WRITE_WALLET: &str = "write:wallet";
pub const PERMISSION_READ_WALLET: &str = "read:wallet";
pub const PERMISSION_READ_TRANSFER: &str = "read:transfer";
pub const PERMISSION_WRITE_TRANSFER: &str = "write:transfer";
pub const PERMISSION_READ_OPERATION: &str = "read:operation";
pub const PERMISSION_WRITE_OPERATION: &str = "write:operation";
pub const PERMISSION_REGISTER_USER: &str = "read:register-user";
pub const PERMISSION_READ_USER: &str = "read:user";
pub const PERMISSION_WRITE_USER: &str = "write:user";

thread_local! {
  /// The list of assets that are supported by the bank canister (e.g. `ICP`, `BTC`, `ETH`, etc.)
  pub static BANK_ASSETS: RefCell<HashSet<BankAsset>> =
      RefCell::new(vec![
        BankAsset {
          blockchain: Blockchain::InternetComputer,
          standards: vec![BlockchainStandard::Native],
          symbol: "ICP".to_string(),
          name: "Internet Computer".to_string(),
          metadata: HashMap::new(),
        },
      ].into_iter().collect());
}

pub fn default_bank_permissions() -> Vec<Permission> {
    vec![
        Permission {
            permission_id: PERMISSION_ADMIN.to_string(),
            access_roles: vec![AccessRole::Admin],
        },
        Permission {
            permission_id: PERMISSION_READ_FEATURES.to_string(),
            access_roles: vec![AccessRole::Admin, AccessRole::User, AccessRole::Guest],
        },
        Permission {
            permission_id: PERMISSION_WRITE_WALLET.to_string(),
            access_roles: vec![AccessRole::Admin, AccessRole::User],
        },
        Permission {
            permission_id: PERMISSION_READ_WALLET.to_string(),
            access_roles: vec![AccessRole::Admin, AccessRole::User],
        },
        Permission {
            permission_id: PERMISSION_READ_TRANSFER.to_string(),
            access_roles: vec![AccessRole::Admin, AccessRole::User],
        },
        Permission {
            permission_id: PERMISSION_WRITE_TRANSFER.to_string(),
            access_roles: vec![AccessRole::Admin, AccessRole::User],
        },
        Permission {
            permission_id: PERMISSION_READ_OPERATION.to_string(),
            access_roles: vec![AccessRole::Admin, AccessRole::User],
        },
        Permission {
            permission_id: PERMISSION_WRITE_OPERATION.to_string(),
            access_roles: vec![AccessRole::Admin, AccessRole::User],
        },
        Permission {
            permission_id: PERMISSION_REGISTER_USER.to_string(),
            access_roles: vec![AccessRole::Admin, AccessRole::User, AccessRole::Guest],
        },
        Permission {
            permission_id: PERMISSION_WRITE_USER.to_string(),
            access_roles: vec![AccessRole::Admin, AccessRole::User],
        },
        Permission {
            permission_id: PERMISSION_READ_USER.to_string(),
            access_roles: vec![AccessRole::Admin, AccessRole::User, AccessRole::Guest],
        },
    ]
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Permission {
    pub permission_id: String,
    pub access_roles: Vec<AccessRole>,
}

#[stable_object(size = WASM_PAGE_SIZE)]
#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct CanisterConfig {
    /// Last time the canister was upgraded or initialized.
    pub last_upgrade_timestamp: Timestamp,
    /// The threshold of approvals required for operations to be executed.
    pub approval_threshold: u8,
    /// The permissions of the canister.
    pub permissions: Vec<Permission>,
    /// The default users of the canister.
    pub owners: Vec<Principal>,
    /// The default wallet policies of the canister,
    /// automatically applied to all wallets if they do not have their own policies.
    pub wallet_policies: Vec<WalletPolicy>,
}

impl Default for CanisterConfig {
    fn default() -> Self {
        Self {
            last_upgrade_timestamp: time(),
            approval_threshold: 100u8,
            permissions: default_bank_permissions(),
            owners: vec![],
            wallet_policies: vec![],
        }
    }
}

impl BankPermissionDTO {
    pub fn to_permission(&self) -> Permission {
        Permission {
            permission_id: self.permission_id.clone(),
            access_roles: self
                .access_roles
                .iter()
                .map(|role| role.to_access_role())
                .collect(),
        }
    }
}

impl Permission {
    pub fn to_dto(&self) -> BankPermissionDTO {
        BankPermissionDTO {
            permission_id: self.permission_id.clone(),
            access_roles: self.access_roles.iter().map(|role| role.to_dto()).collect(),
        }
    }
}

impl UserRoleDTO {
    pub fn to_access_role(&self) -> AccessRole {
        match self {
            UserRoleDTO::Admin => AccessRole::Admin,
            UserRoleDTO::User => AccessRole::User,
            UserRoleDTO::Guest => AccessRole::Guest,
        }
    }
}

impl AccessRole {
    pub fn to_dto(&self) -> UserRoleDTO {
        match self {
            AccessRole::Admin => UserRoleDTO::Admin,
            AccessRole::User => UserRoleDTO::User,
            AccessRole::Guest => UserRoleDTO::Guest,
        }
    }
}

impl CanisterConfig {
    /// The maximum size of each field in stable memory.
    pub const MAX_BYTE_SIZE_LAST_UPGRADE_TIMESTAMP: u32 = std::mem::size_of::<u64>() as u32;
    pub const MAX_BYTE_SIZE_APPROVAL_THRESHOLD: u32 = std::mem::size_of::<u8>() as u32;

    /// The maximum size of the CanisterConfig in stable memory.
    pub const MAX_BYTE_SIZE: u32 = WASM_PAGE_SIZE;

    /// If this overflows then the stable memory layout will be broken.
    pub const SPARE_BYTES: u32 = Self::MAX_BYTE_SIZE
        - Self::MAX_BYTE_SIZE_LAST_UPGRADE_TIMESTAMP
        - Self::MAX_BYTE_SIZE_APPROVAL_THRESHOLD;
}

/// Configuration state of the canister.
pub enum CanisterState {
    Uninitialized, // This state is only used between wasm module initialization and init().
    Initialized(CanisterConfig),
}

impl CanisterState {
    pub fn get(&self) -> &CanisterConfig {
        match &self {
            CanisterState::Uninitialized => trap("canister config not initialized"),
            CanisterState::Initialized(config) => config,
        }
    }
}

/// Adds serialization and deserialization support of CanisterState to stable memory.
impl Storable for CanisterState {
    fn to_bytes(&self) -> Cow<[u8]> {
        match &self {
            CanisterState::Uninitialized => Cow::Borrowed(&[]),
            CanisterState::Initialized(config) => config.to_bytes(),
        }
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        if bytes.len() == 0 {
            return CanisterState::Uninitialized;
        }
        CanisterState::Initialized(CanisterConfig::from_bytes(bytes))
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: WASM_PAGE_SIZE,
        is_fixed_size: false,
    };
}

#[cfg(test)]
pub mod test_utils {
    use crate::core::{write_canister_config, CanisterConfig};

    pub fn init_canister_config() -> CanisterConfig {
        let config = CanisterConfig::default();
        write_canister_config(config.clone());

        config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn permission_admin_only_has_admin_role() {
        let permissions = default_bank_permissions();
        let admin_permission = permissions
            .iter()
            .find(|p| p.permission_id == PERMISSION_ADMIN)
            .unwrap_or_else(|| panic!("Admin permission not found"));

        assert_eq!(admin_permission.access_roles.len(), 1);
        assert!(admin_permission.access_roles.contains(&AccessRole::Admin));
    }
}
