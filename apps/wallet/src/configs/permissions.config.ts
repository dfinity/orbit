import { Resource } from '~/generated/station/station.did';

/**
 * Adds type safety to the permissions object to avoid defining permission keys multiple times.
 *
 * @param permissions The permissions to define
 * @returns Type safe permissions
 */
function definePermissions<T extends Record<string, Resource>>(permissions: T): T {
  return permissions;
}

/**
 * Global permissions associated with resource actions.
 *
 * Note that these are not tied to a specific resource id (e.g. a specific account id).
 */
export const GLOBAL_PERMISSIONS = {
  treasury: definePermissions({
    list_accounts: { Account: { List: null } },
    create_account: { Account: { Create: null } },
    read_any_account: { Account: { Read: { Any: null } } },
    update_any_account: { Account: { Update: { Any: null } } },
    transfer_from_any_account: { Account: { Transfer: { Any: null } } },
    list_addressbook: { AddressBook: { List: null } },
    read_any_addressbook: { AddressBook: { Read: { Any: null } } },
    create_addressbook: { AddressBook: { Create: null } },
    update_any_addressbook: { AddressBook: { Update: { Any: null } } },
    delete_any_addressbook: { AddressBook: { Delete: { Any: null } } },
    list_assets: { Asset: { List: null } },
    create_asset: { Asset: { Create: null } },
    read_any_asset: { Asset: { Read: { Any: null } } },
    update_any_asset: { Asset: { Update: { Any: null } } },
    delete_any_asset: { Asset: { Delete: { Any: null } } },
  }),
  canisters: definePermissions({
    list_external_canisters: { ExternalCanister: { List: null } },
    create_external_canister: { ExternalCanister: { Create: null } },
    read_any_external_canister: { ExternalCanister: { Read: { Any: null } } },
    change_any_external_canister: { ExternalCanister: { Change: { Any: null } } },
    fund_any_external_canister: { ExternalCanister: { Fund: { Any: null } } },
    call_any_external_canister: {
      ExternalCanister: {
        Call: { execution_method: { Any: null }, validation_method: { No: null } },
      },
    },
  }),
  users: definePermissions({
    list_users: { User: { List: null } },
    create_user: { User: { Create: null } },
    read_any_user: { User: { Read: { Any: null } } },
    update_any_user: { User: { Update: { Any: null } } },
    list_usergroups: { UserGroup: { List: null } },
    create_usergroup: { UserGroup: { Create: null } },
    read_any_usergroup: { UserGroup: { Read: { Any: null } } },
    update_any_usergroup: { UserGroup: { Update: { Any: null } } },
    delete_any_usergroup: { UserGroup: { Delete: { Any: null } } },
  }),
  system: definePermissions({
    capabilities: { System: { Capabilities: null } },
    read_system_info: { System: { SystemInfo: null } },
    manage_system_info: { System: { ManageSystemInfo: null } },
    system_upgrade: { System: { Upgrade: null } },
    manage_permissions: { Permission: { Update: null } },
    list_request_policies: { RequestPolicy: { List: null } },
    create_request_policy: { RequestPolicy: { Create: null } },
    read_any_request_policy: { RequestPolicy: { Read: { Any: null } } },
    update_any_request_policy: { RequestPolicy: { Update: { Any: null } } },
    delete_any_request_policy: { RequestPolicy: { Delete: { Any: null } } },
    list_requests: { Request: { List: null } },
    read_any_request: { Request: { Read: { Any: null } } },
  }),
};
