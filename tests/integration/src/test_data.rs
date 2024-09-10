//! Helper functions to generate test data for the integration tests.

use candid::Principal;
use pocket_ic::PocketIc;
use system_upgrade::perform_upgrader_update;

use crate::setup::get_canister_wasm;

pub mod account;
pub mod address_book;
pub mod asset;
pub mod permission;
pub mod request_policy;
pub mod system_upgrade;
pub mod user;
pub mod user_group;

thread_local! {
  static UNIQUE_ID: std::cell::RefCell<u64> = const { std::cell::RefCell::new(0) };
}

/// Force the next generated test data ID to the given value.
pub fn set_test_data_id(next: u64) {
    UNIQUE_ID.with(|counter| {
        let mut counter = counter.borrow_mut();
        *counter = next;
    });
}

/// Generate an ID for test data.
///
/// Every time this function is called, it will return a new unique ID.
pub fn next_unique_id() -> u64 {
    UNIQUE_ID.with(|counter| {
        let mut counter = counter.borrow_mut();
        *counter += 1;
        *counter
    })
}

pub struct StationDataGenerator<'a> {
    env: &'a PocketIc,
    station_canister_id: Principal,
    requester: Principal,
    perform_edit_operations: bool,
    count_requests: usize,
    max_user_groups_per_user: usize,
    has_generated: bool,
    // Number of entries to generate for each type
    users: usize,
    user_groups: usize,
    accounts: usize,
    address_book_entries: usize,
    upgrader_updates: usize,
    station_updates: usize,
    permission_updates: usize,
    request_policy_updates: usize,
    assets: usize,
}

impl<'a> StationDataGenerator<'a> {
    const DEFAULT_ENTRIES: usize = 5;

    pub fn new(env: &'a PocketIc, station_canister_id: Principal, requester: Principal) -> Self {
        Self {
            env,
            station_canister_id,
            requester,
            perform_edit_operations: false,
            users: Self::DEFAULT_ENTRIES,
            user_groups: Self::DEFAULT_ENTRIES,
            accounts: Self::DEFAULT_ENTRIES,
            address_book_entries: Self::DEFAULT_ENTRIES,
            upgrader_updates: 1,
            station_updates: 1,
            permission_updates: 1,
            request_policy_updates: 1,
            max_user_groups_per_user: 5,
            has_generated: false,
            count_requests: 0,
            assets: Self::DEFAULT_ENTRIES,
        }
    }

    pub fn request_count(&self) -> usize {
        self.count_requests
    }

    pub fn with_upgrader_updates(mut self, upgrader_updates: usize) -> Self {
        self.upgrader_updates = upgrader_updates;
        self
    }

    pub fn with_station_updates(mut self, station_updates: usize) -> Self {
        self.station_updates = station_updates;
        self
    }

    pub fn with_permission_updates(mut self, permission_updates: usize) -> Self {
        self.permission_updates = permission_updates;
        self
    }

    pub fn with_request_policy_updates(mut self, request_policy_updates: usize) -> Self {
        self.request_policy_updates = request_policy_updates;
        self
    }

    pub fn with_users(mut self, users: usize) -> Self {
        self.users = users;
        self
    }

    pub fn with_user_groups(mut self, user_groups: usize) -> Self {
        self.user_groups = user_groups;
        self
    }

    pub fn with_accounts(mut self, accounts: usize) -> Self {
        self.accounts = accounts;
        self
    }

    pub fn with_address_book_entries(mut self, address_book_entries: usize) -> Self {
        self.address_book_entries = address_book_entries;
        self
    }

    pub fn with_assets(mut self, assets: usize) -> Self {
        self.assets = assets;
        self
    }

    pub fn with_edit_operations(mut self) -> Self {
        self.perform_edit_operations = true;
        self
    }

    pub fn with_max_user_groups_per_user(mut self, max_user_groups_per_user: usize) -> Self {
        self.max_user_groups_per_user = max_user_groups_per_user;
        self
    }

    fn increment_request_count(&mut self) {
        self.count_requests += 1;
    }

    pub fn generate(&mut self) {
        if self.has_generated {
            panic!("Test data has already been generated");
        }

        self.has_generated = true;

        // Add the user groups
        let mut user_groups = Vec::new();
        for _ in 0..self.user_groups {
            let user_group =
                user_group::add_user_group(self.env, self.station_canister_id, self.requester);
            self.increment_request_count();

            if self.perform_edit_operations {
                user_group::edit_user_group(
                    self.env,
                    self.station_canister_id,
                    self.requester,
                    user_group.id.clone(),
                    format!("{}_edited", user_group.name),
                );
                self.increment_request_count();
            }

            user_groups.push(user_group);
        }

        // Add the users
        let mut user_group_ids = user_groups
            .iter()
            .map(|group| group.id.clone())
            .collect::<Vec<_>>();

        user_group_ids.truncate(self.max_user_groups_per_user);

        for _ in 0..self.users {
            let user = user::add_user(
                self.env,
                self.station_canister_id,
                self.requester,
                user_group_ids.clone(),
            );
            self.increment_request_count();

            if self.perform_edit_operations {
                user::edit_user_name(
                    self.env,
                    self.station_canister_id,
                    self.requester,
                    user.id.clone(),
                    format!("{}_edited", user.name),
                );
                self.increment_request_count();
            }
        }

        // Add the accounts
        for _ in 0..self.accounts {
            let account = account::add_account(self.env, self.station_canister_id, self.requester);
            self.increment_request_count();

            if self.perform_edit_operations {
                account::edit_account_name(
                    self.env,
                    self.station_canister_id,
                    self.requester,
                    account.id.clone(),
                    format!("{}_edited", account.name),
                );
                self.increment_request_count();
            }
        }

        // Add the address book entries
        for _ in 0..self.address_book_entries {
            let address_book_entry = address_book::add_address_book_entry(
                self.env,
                self.station_canister_id,
                self.requester,
            );
            self.increment_request_count();

            if self.perform_edit_operations {
                address_book::edit_address_book_entry_owner(
                    self.env,
                    self.station_canister_id,
                    self.requester,
                    address_book_entry.id.clone(),
                    format!("{}_edited", address_book_entry.address_owner),
                );
                self.increment_request_count();
            }
        }

        // Add the assets
        for _ in 0..self.assets {
            let asset = asset::add_asset(self.env, self.station_canister_id, self.requester);
            self.increment_request_count();

            if self.perform_edit_operations {
                asset::edit_asset_name(
                    self.env,
                    self.station_canister_id,
                    self.requester,
                    asset.id.clone(),
                    format!("{}_edited", asset.name),
                );
                self.increment_request_count();
            }
        }

        // Edit the permissions
        for _ in 0..self.permission_updates {
            permission::edit_permission(
                self.env,
                self.station_canister_id,
                self.requester,
                station_api::ResourceDTO::System(station_api::SystemResourceActionDTO::SystemInfo),
            );
            self.increment_request_count();
        }

        // Add new request policies
        for _ in 0..self.request_policy_updates {
            request_policy::add_request_policy(
                self.env,
                self.station_canister_id,
                self.requester,
                station_api::RequestSpecifierDTO::AddAccount,
            );
            self.increment_request_count();
        }

        let station_wasm = get_canister_wasm("station").to_vec();
        let upgrader_wasm = get_canister_wasm("upgrader").to_vec();

        // Perform the upgrader updates
        for _ in 0..self.upgrader_updates {
            perform_upgrader_update(
                self.env,
                self.station_canister_id,
                self.requester,
                upgrader_wasm.clone(),
            );
            self.increment_request_count();
        }

        // Perform the station updates
        for _ in 0..self.station_updates {
            system_upgrade::perform_station_update(
                self.env,
                self.station_canister_id,
                self.requester,
                station_wasm.clone(),
            );
            self.increment_request_count();
        }
    }
}
