use super::WASM_PAGE_SIZE;
use crate::{
    models::{AccessRole, WalletPolicy},
    transport::{AccountRoleDTO, BankPermissionDTO},
};
use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use ic_canister_core::{
    cdk::api::{time, trap},
    types::Timestamp,
};
use ic_canister_macros::stable_object;
use ic_stable_structures::Storable;
use std::borrow::Cow;

/// The list of permissions that can be granted to roles, admin role has all permissions.
pub const PERMISSION_ADMIN: &str = "admin";
pub const PERMISSION_READ_FEATURES: &str = "read:features";
pub const PERMISSION_WRITE_WALLET: &str = "write:wallet";
pub const PERMISSION_READ_WALLET: &str = "read:wallet";
pub const PERMISSION_READ_TRANSFER: &str = "read:transfer";
pub const PERMISSION_WRITE_TRANSFER: &str = "write:transfer";
pub const PERMISSION_READ_OPERATION: &str = "read:operation";
pub const PERMISSION_WRITE_OPERATION: &str = "write:operation";

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
    ]
}

#[stable_object(size = 96)]
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
    /// The default accounts of the canister.
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

impl AccountRoleDTO {
    pub fn to_access_role(&self) -> AccessRole {
        match self {
            AccountRoleDTO::Admin => AccessRole::Admin,
            AccountRoleDTO::User => AccessRole::User,
            AccountRoleDTO::Guest => AccessRole::Guest,
        }
    }
}

impl AccessRole {
    pub fn to_dto(&self) -> AccountRoleDTO {
        match self {
            AccessRole::Admin => AccountRoleDTO::Admin,
            AccessRole::User => AccountRoleDTO::User,
            AccessRole::Guest => AccountRoleDTO::Guest,
        }
    }
}

impl CanisterConfig {
    /// The maximum size of each field in stable memory.
    pub const MAX_BYTE_SIZE_LAST_UPGRADE_TIMESTAMP: u32 = std::mem::size_of::<u64>() as u32;
    pub const MAX_BYTE_SIZE_APPROVAL_THRESHOLD: u32 = std::mem::size_of::<u8>() as u32;

    /// The maximum size of the CanisterConfig in stable memory.
    pub const MAX_BYTE_SIZE: u32 = WASM_PAGE_SIZE;

    /// The number of bytes that are not used by the account and could be used to add more fields to the account
    /// without breaking the stable memory layout, if this overflows then the stable memory layout will be broken.
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
}
