use std::cell::RefCell;

use candid::Principal;
use ic_stable_structures::memory_manager::MemoryId;
use orbit_essentials::{storable, types::UUID};
use upgrader_api::MetadataDTO;
use uuid::Uuid;

use crate::{helper::HelperMapper, StableValue, MEMORY_ID_DISASTER_RECOVERY_ID, MEMORY_MANAGER};

thread_local! {

    static STORAGE: RefCell<StableValue<DisasterRecovery>> = RefCell::new(
        StableValue::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(MEMORY_ID_DISASTER_RECOVERY_ID))),
        )
    );

}

#[storable]
#[derive(Clone, Debug)]
pub struct DisasterRecoveryCommittee {
    pub users: Vec<AdminUser>,
    pub quorum_percentage: u16,
}

impl From<upgrader_api::DisasterRecoveryCommittee> for DisasterRecoveryCommittee {
    fn from(value: upgrader_api::DisasterRecoveryCommittee) -> Self {
        DisasterRecoveryCommittee {
            users: value.users.into_iter().map(AdminUser::from).collect(),
            quorum_percentage: value.quorum_percentage,
        }
    }
}

impl From<DisasterRecoveryCommittee> for upgrader_api::DisasterRecoveryCommittee {
    fn from(value: DisasterRecoveryCommittee) -> Self {
        upgrader_api::DisasterRecoveryCommittee {
            users: value
                .users
                .into_iter()
                .map(upgrader_api::AdminUser::from)
                .collect(),
            quorum_percentage: value.quorum_percentage,
        }
    }
}

#[storable]
#[derive(Clone, Debug)]
pub struct Metadata {
    pub key: String,
    pub value: String,
}

impl From<upgrader_api::MetadataDTO> for Metadata {
    fn from(value: MetadataDTO) -> Self {
        Metadata {
            key: value.key,
            value: value.value,
        }
    }
}

impl From<Metadata> for upgrader_api::MetadataDTO {
    fn from(value: Metadata) -> Self {
        upgrader_api::MetadataDTO {
            key: value.key,
            value: value.value,
        }
    }
}

#[storable]
#[derive(Clone, Debug)]
pub struct AdminUser {
    /// The user ID.
    pub id: UUID,
    /// The name of the user (if any).
    pub name: String,
    /// The identities associated with the user.
    pub identities: Vec<Principal>,
}

impl From<upgrader_api::AdminUser> for AdminUser {
    fn from(value: upgrader_api::AdminUser) -> Self {
        AdminUser {
            id: *HelperMapper::to_uuid(value.id)
                .expect("Invalid user ID")
                .as_bytes(),
            name: value.name,
            identities: value.identities,
        }
    }
}

impl From<AdminUser> for upgrader_api::AdminUser {
    fn from(value: AdminUser) -> Self {
        upgrader_api::AdminUser {
            id: Uuid::from_bytes(value.id).hyphenated().to_string(),
            name: value.name,
            identities: value.identities,
        }
    }
}

#[storable]
#[derive(Clone, Debug)]
pub struct Account {
    /// The account id, which is a UUID.
    pub id: UUID,
    /// The blockchain type (e.g. `icp`, `eth`, `btc`)
    pub blockchain: String,
    /// The account address (e.g. `0x1234`, etc.)
    pub address: String,
    /// The blockchain standard (e.g. `native`, `icrc1`, `erc20`, etc.)
    pub standard: String,
    /// The asset symbol (e.g. `ICP`, `ETH`, `BTC`, etc.)
    pub symbol: String,
    /// The asset decimals (e.g. `8` for `BTC`, `18` for `ETH`, etc.)
    pub decimals: u32,
    /// The account name (e.g. `My Main Account`)
    pub name: String,
    /// The account metadata, which is a list of key-value pairs,
    /// where the key is unique and the first entry in the tuple,
    /// and the value is the second entry in the tuple.
    pub metadata: Vec<Metadata>,
}

impl From<upgrader_api::Account> for Account {
    fn from(value: upgrader_api::Account) -> Self {
        Account {
            id: *HelperMapper::to_uuid(value.id)
                .expect("Invalid account ID")
                .as_bytes(),
            blockchain: value.blockchain,
            address: value.address,
            standard: value.standard,
            symbol: value.symbol,
            decimals: value.decimals,
            name: value.name,
            metadata: value.metadata.into_iter().map(Metadata::from).collect(),
        }
    }
}

impl From<Account> for upgrader_api::Account {
    fn from(value: Account) -> Self {
        upgrader_api::Account {
            id: Uuid::from_bytes(value.id).hyphenated().to_string(),
            blockchain: value.blockchain,
            address: value.address,
            standard: value.standard,
            symbol: value.symbol,
            decimals: value.decimals,
            name: value.name,
            metadata: value
                .metadata
                .into_iter()
                .map(upgrader_api::MetadataDTO::from)
                .collect(),
        }
    }
}

#[storable]
#[derive(Clone, Debug, Default)]
pub struct DisasterRecovery {
    pub accounts: Vec<Account>,
    pub committee: Option<DisasterRecoveryCommittee>,
}

impl DisasterRecovery {
    pub fn get() -> DisasterRecovery {
        STORAGE.with(|storage| storage.borrow().get(&()).unwrap_or_default())
    }

    fn set(value: DisasterRecovery) {
        STORAGE.with(|storage| storage.borrow_mut().insert((), value));
    }

    pub fn set_committee(committee: DisasterRecoveryCommittee) {
        let mut value = Self::get();
        value.committee = Some(committee);

        Self::set(value);
    }

    pub fn set_accounts(accounts: Vec<Account>) {
        let mut value = Self::get();
        value.accounts = accounts;
        Self::set(value);
    }

    pub fn is_committee_member(principal: &Principal) -> bool {
        Self::get().committee.as_ref().map_or(false, |committee| {
            committee
                .users
                .iter()
                .any(|user| user.identities.contains(principal))
        })
    }
}
