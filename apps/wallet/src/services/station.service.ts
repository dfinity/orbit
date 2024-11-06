import { Actor, ActorSubclass, HttpAgent } from '@dfinity/agent';
import { Principal } from '@dfinity/principal';
import { idlFactory } from '~/generated/station';
import {
  Account,
  AccountBalance,
  AccountCallerPrivileges,
  AddAccountOperationInput,
  AddAddressBookEntryOperationInput,
  AddAssetOperationInput,
  AddRequestPolicyOperationInput,
  AddUserGroupOperationInput,
  AddUserOperationInput,
  CanisterMethod,
  CanisterStatusResult,
  Capabilities,
  ChangeExternalCanisterOperationInput,
  ConfigureExternalCanisterOperationKind,
  ConfigureExternalCanisterSettingsInput,
  CreateExternalCanisterOperationInput,
  CreateRequestInput,
  DefiniteCanisterSettingsInput,
  DisasterRecoveryCommittee,
  EditAccountOperationInput,
  EditAddressBookEntryOperationInput,
  EditAssetOperationInput,
  EditPermissionOperationInput,
  EditRequestPolicyOperationInput,
  EditUserGroupOperationInput,
  EditUserOperationInput,
  FetchAccountBalancesInput,
  FundExternalCanisterOperationInput,
  GetAccountInput,
  GetAccountResult,
  GetAddressBookEntryInput,
  GetAddressBookEntryResult,
  GetAssetInput,
  GetAssetResult,
  GetExternalCanisterFiltersResult,
  GetExternalCanisterResult,
  GetNextApprovableRequestResult,
  GetPermissionInput,
  GetPermissionResult,
  GetRequestInput,
  GetRequestPolicyResult,
  GetRequestResult,
  GetTransfersInput,
  GetUserGroupInput,
  GetUserGroupResult,
  GetUserInput,
  GetUserResult,
  ListAccountTransfersInput,
  ListAccountsResult,
  ListAddressBookEntriesResult,
  ListAssetsResult,
  ListExternalCanistersResult,
  ListNotificationsInput,
  ListPermissionsInput,
  ListPermissionsResult,
  ListRequestPoliciesResult,
  ListRequestsInput,
  ListRequestsResult,
  ListUserGroupsResult,
  ListUsersResult,
  ManageSystemInfoOperationInput,
  MarkNotificationsReadInput,
  Notification,
  PaginationInput,
  RemoveAssetOperationInput,
  RemoveUserGroupOperationInput,
  Request,
  SubmitRequestApprovalInput,
  SystemInfoResult,
  SystemUpgradeOperationInput,
  Transfer,
  TransferListItem,
  TransferOperationInput,
  UUID,
  User,
  UserPrivilege,
  UserStatus,
  _SERVICE,
} from '~/generated/station/station.did';
import { ExtractOk } from '~/types/helper.types';
import {
  GetNextApprovableRequestArgs,
  ListAccountsArgs,
  ListAddressBookEntriesArgs,
  ListAssetsArgs,
  ListExternalCanistersArgs,
  ListRequestsArgs,
} from '~/types/station.types';
import { transformIdlWithOnlyVerifiedCalls, variantIs } from '~/utils/helper.utils';

export class StationService {
  // This actor is modified to only perform calls that can be verified, such as update calls that go through consensus.
  private verified_actor: ActorSubclass<_SERVICE>;

  // This is the default actor that can perform all calls, including query calls.
  private actor: ActorSubclass<_SERVICE>;

  public static ERR_USER_IDENTITY_NOT_FOUND = 'NOT_FOUND_USER_IDENTITY';
  public static ERR_USER_NOT_FOUND = 'NOT_FOUND';

  constructor(
    private agent: HttpAgent,
    private stationId: Principal = Principal.anonymous(),
  ) {
    this.actor = Actor.createActor<_SERVICE>(idlFactory, {
      agent: this.agent,
      canisterId: stationId,
    });

    this.verified_actor = Actor.createActor<_SERVICE>(
      transformIdlWithOnlyVerifiedCalls(idlFactory),
      {
        agent: this.agent,
        canisterId: stationId,
      },
    );
  }

  getStationId() {
    return this.stationId;
  }

  withStationId(stationId: Principal): StationService {
    this.stationId = stationId;
    this.actor = Actor.createActor<_SERVICE>(idlFactory, {
      agent: this.agent,
      canisterId: stationId,
    });

    this.verified_actor = Actor.createActor<_SERVICE>(
      transformIdlWithOnlyVerifiedCalls(idlFactory),
      {
        agent: this.agent,
        canisterId: stationId,
      },
    );

    return this;
  }

  async getUser(input: GetUserInput, verifiedCall = false): Promise<ExtractOk<GetUserResult>> {
    const actor = verifiedCall ? this.verified_actor : this.actor;
    const result = await actor.get_user(input);

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok;
  }

  async getPermission(
    input: GetPermissionInput,
    verifiedCall = false,
  ): Promise<ExtractOk<GetPermissionResult>> {
    const actor = verifiedCall ? this.verified_actor : this.actor;
    const result = await actor.get_permission(input);

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok;
  }

  async getAccountPermissions(
    accountId: UUID,
    verifiedCall = false,
  ): Promise<ExtractOk<ListPermissionsResult>> {
    const actor = verifiedCall ? this.verified_actor : this.actor;

    const result = await actor.list_permissions({
      resources: [
        [
          { Account: { Read: { Id: accountId } } },
          { Account: { Transfer: { Id: accountId } } },
          { Account: { Update: { Id: accountId } } },
        ],
      ],
      paginate: [
        {
          limit: [3],
          offset: [],
        },
      ],
    });

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok;
  }

  async getUserGroup(
    input: GetUserGroupInput,
    verifiedCall = false,
  ): Promise<ExtractOk<GetUserGroupResult>> {
    const actor = verifiedCall ? this.verified_actor : this.actor;
    const result = await actor.get_user_group(input);
    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok;
  }

  async myUser(verifiedCall = false): Promise<{ me: User; privileges: UserPrivilege[] } | null> {
    const actor = verifiedCall ? this.verified_actor : this.actor;
    const result = await actor.me();
    if (variantIs(result, 'Err')) {
      if (result.Err.code === StationService.ERR_USER_NOT_FOUND) {
        return null;
      }

      throw result.Err;
    }

    return {
      me: result.Ok.me,
      privileges: result.Ok.privileges,
    };
  }

  async listUserGroups(
    {
      limit,
      offset,
      searchTerm,
    }: {
      limit?: number;
      offset?: number;
      searchTerm?: string;
    } = {},
    verifiedCall = false,
  ): Promise<ExtractOk<ListUserGroupsResult>> {
    const actor = verifiedCall ? this.verified_actor : this.actor;
    const result = await actor.list_user_groups({
      search_term: searchTerm ? [searchTerm] : [],
      paginate:
        limit || offset
          ? [
              {
                limit: limit !== undefined ? [limit] : [],
                offset: offset !== undefined ? [BigInt(offset)] : [],
              },
            ]
          : [],
    });

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok;
  }

  async removeUserGroup(input: RemoveUserGroupOperationInput): Promise<Request> {
    const result = await this.actor.create_request({
      execution_plan: [{ Immediate: null }],
      title: [],
      summary: [],
      operation: { RemoveUserGroup: input },
    });

    if ('Err' in result) {
      throw result.Err;
    }

    return result.Ok.request;
  }

  async addUserGroup(input: AddUserGroupOperationInput): Promise<Request> {
    const result = await this.actor.create_request({
      execution_plan: [{ Immediate: null }],
      title: [],
      summary: [],
      operation: { AddUserGroup: input },
    });

    if ('Err' in result) {
      throw result.Err;
    }

    return result.Ok.request;
  }

  async editUserGroup(input: EditUserGroupOperationInput): Promise<Request> {
    const result = await this.actor.create_request({
      execution_plan: [{ Immediate: null }],
      title: [],
      summary: [],
      operation: { EditUserGroup: input },
    });

    if ('Err' in result) {
      throw result.Err;
    }

    return result.Ok.request;
  }

  async addUser(input: AddUserOperationInput): Promise<Request> {
    const result = await this.actor.create_request({
      execution_plan: [{ Immediate: null }],
      title: [],
      summary: [],
      operation: { AddUser: input },
    });

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok.request;
  }

  async editUser(input: EditUserOperationInput): Promise<Request> {
    const result = await this.actor.create_request({
      execution_plan: [{ Immediate: null }],
      title: [],
      summary: [],
      operation: { EditUser: input },
    });

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok.request;
  }

  async listUsers(
    {
      limit,
      offset,
      searchTerm,
      statuses,
      groups,
    }: {
      limit?: number;
      offset?: number;
      searchTerm?: string;
      statuses?: UserStatus[];
      groups?: UUID[];
    } = {},
    verifiedCall = false,
  ): Promise<ExtractOk<ListUsersResult>> {
    const actor = verifiedCall ? this.verified_actor : this.actor;
    const result = await actor.list_users({
      paginate:
        limit || offset
          ? [
              {
                limit: limit ? [limit] : [],
                offset: offset ? [BigInt(offset)] : [],
              },
            ]
          : [],
      statuses: statuses ? [statuses] : [],
      search_term: searchTerm ? [searchTerm] : [],
      groups: groups ? [groups] : [],
    });

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok;
  }

  async capabilities(verifiedCall = false): Promise<Capabilities> {
    const actor = verifiedCall ? this.verified_actor : this.actor;
    const result = await actor.capabilities();

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok.capabilities;
  }

  async systemInfo(verifiedCall = false): Promise<ExtractOk<SystemInfoResult>> {
    const actor = verifiedCall ? this.verified_actor : this.actor;
    const result = await actor.system_info();

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok;
  }

  async listNotifications(
    input: ListNotificationsInput,
    verifiedCall = false,
  ): Promise<Notification[]> {
    const actor = verifiedCall ? this.verified_actor : this.actor;
    const result = await actor.list_notifications(input);

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok.notifications;
  }

  async listRequests(
    {
      created_dt,
      expiration_dt,
      limit,
      offset,
      requesterIds,
      statuses,
      types,
      approverIds,
      sortBy,
      onlyApprovable,
    }: ListRequestsArgs = {},
    verifiedCall = false,
  ): Promise<ExtractOk<ListRequestsResult>> {
    const actor = verifiedCall ? this.verified_actor : this.actor;
    const paginate: PaginationInput = {
      limit: limit ? [limit] : [],
      offset: offset ? [BigInt(offset)] : [],
    };

    let sortingCriteria: ListRequestsInput['sort_by'] | [] = [];
    if (sortBy && variantIs(sortBy, 'createdAt')) {
      sortingCriteria = [
        { CreatedAt: sortBy.createdAt === 'asc' ? { Asc: null } : { Desc: null } },
      ];
    } else if (sortBy && variantIs(sortBy, 'expirationDt')) {
      sortingCriteria = [
        { ExpirationDt: sortBy.expirationDt === 'asc' ? { Asc: null } : { Desc: null } },
      ];
    } else if (sortBy && variantIs(sortBy, 'lastModified')) {
      sortingCriteria = [
        { LastModificationDt: sortBy.lastModified === 'asc' ? { Asc: null } : { Desc: null } },
      ];
    }

    const result = await actor.list_requests({
      statuses: statuses ? [statuses] : [],
      created_from_dt: created_dt?.fromDt ? [created_dt.fromDt.toISOString()] : [],
      created_to_dt: created_dt?.toDt ? [created_dt.toDt.toISOString()] : [],
      expiration_from_dt: expiration_dt?.fromDt ? [expiration_dt.fromDt.toISOString()] : [],
      expiration_to_dt: expiration_dt?.toDt ? [expiration_dt.toDt.toISOString()] : [],
      operation_types: types ? [types] : [],
      requester_ids: requesterIds ? [requesterIds] : [],
      approver_ids: approverIds ? [approverIds] : [],
      paginate: [paginate],
      sort_by: sortingCriteria,
      only_approvable: !!onlyApprovable,
      with_evaluation_results: false,
    });

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok;
  }

  async getNextApprovableRequest(
    { types, excludedRequestIds }: GetNextApprovableRequestArgs = {},
    verifiedCall = false,
  ): Promise<ExtractOk<GetNextApprovableRequestResult>> {
    const actor = verifiedCall ? this.verified_actor : this.actor;
    const result = await actor.get_next_approvable_request({
      operation_types: types ? [types] : [],
      excluded_request_ids: excludedRequestIds ?? [],
    });

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok;
  }

  async listUnreadNotifications(
    from_dt?: Date,
    last_id?: UUID,
    verifiedCall = false,
  ): Promise<Notification[]> {
    const notifications = await this.listNotifications(
      {
        notification_type: [],
        status: [{ Sent: null }],
        from_dt: from_dt ? [from_dt.toISOString()] : [],
        to_dt: [],
      },
      verifiedCall,
    );

    return notifications.filter(notification => notification.id !== last_id);
  }

  async markNotificationAsRead(input: MarkNotificationsReadInput): Promise<void> {
    await this.actor.mark_notifications_read(input);
  }

  async submitRequestApproval(input: SubmitRequestApprovalInput): Promise<Request> {
    const result = await this.actor.submit_request_approval(input);

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok.request;
  }

  async getRequest(
    input: GetRequestInput,
    verifiedCall = false,
  ): Promise<ExtractOk<GetRequestResult>> {
    const actor = verifiedCall ? this.verified_actor : this.actor;
    const result = await actor.get_request(input);

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok;
  }

  async listAccounts(
    { limit, offset, searchTerm }: ListAccountsArgs = {},
    verifiedCall = false,
  ): Promise<ExtractOk<ListAccountsResult>> {
    const actor = verifiedCall ? this.verified_actor : this.actor;
    const result = await actor.list_accounts({
      paginate: [
        {
          limit: limit !== undefined ? [limit] : [],
          offset: offset !== undefined ? [BigInt(offset)] : [],
        },
      ],
      search_term: searchTerm ? [searchTerm] : [],
    });

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok;
  }

  async listAllAccounts(verifiedCall = false): Promise<{
    accounts: Account[];
    privileges: AccountCallerPrivileges[];
  }> {
    const actor = verifiedCall ? this.verified_actor : this.actor;

    const accounts: Account[] = [];
    const privileges: AccountCallerPrivileges[] = [];
    let nextOffset: [bigint] | [] = [];

    do {
      const result = await actor.list_accounts({
        paginate: [
          {
            limit: [100],
            offset: nextOffset,
          },
        ],
        search_term: [],
      });

      if (variantIs(result, 'Err')) {
        throw result.Err;
      }

      accounts.push(...result.Ok.accounts);
      privileges.push(...result.Ok.privileges);

      nextOffset = result.Ok.next_offset as [bigint] | []; // have to force cast here because of typescript inference
    } while (nextOffset.length > 0);

    return { accounts, privileges };
  }

  async listAddressBook(
    { limit, offset, blockchain, labels, ids, addresses }: ListAddressBookEntriesArgs = {},
    verifiedCall = false,
  ): Promise<ExtractOk<ListAddressBookEntriesResult>> {
    const actor = verifiedCall ? this.verified_actor : this.actor;
    const result = await actor.list_address_book_entries({
      paginate: [
        {
          limit: limit !== undefined ? [limit] : [],
          offset: offset !== undefined ? [BigInt(offset)] : [],
        },
      ],
      blockchain: blockchain ? [blockchain] : [],
      labels: labels ? [labels] : [],
      addresses: addresses ? [addresses] : [],
      ids: ids ? [ids] : [],
    });

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok;
  }

  async getAsset(input: GetAssetInput, verifiedCall = false): Promise<ExtractOk<GetAssetResult>> {
    const actor = verifiedCall ? this.verified_actor : this.actor;
    const result = await actor.get_asset(input);

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok;
  }

  async fundExternalCanister(input: FundExternalCanisterOperationInput): Promise<Request> {
    const result = await this.actor.create_request({
      execution_plan: [{ Immediate: null }],
      title: [],
      summary: [],
      operation: {
        FundExternalCanister: input,
      },
    });

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok.request;
  }

  async editCanisterIcSettings(
    canisterId: Principal,
    input: DefiniteCanisterSettingsInput,
  ): Promise<Request> {
    const result = await this.actor.create_request({
      execution_plan: [{ Immediate: null }],
      title: [],
      summary: [],
      operation: {
        ConfigureExternalCanister: {
          canister_id: canisterId,
          kind: { NativeSettings: input },
        },
      },
    });

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok.request;
  }

  async changeExternalCanister(
    input: ChangeExternalCanisterOperationInput,
    opts: { comment?: string } = {},
  ): Promise<Request> {
    const result = await this.actor.create_request({
      execution_plan: [{ Immediate: null }],
      title: [],
      summary: opts.comment ? [opts.comment] : [],
      operation: {
        ChangeExternalCanister: input,
      },
    });

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok.request;
  }

  async unlinkExternalCanister(input: {
    canisterId: Principal;
    softDelete?: boolean;
  }): Promise<Request> {
    const shouldSoftDelete = input.softDelete ?? true;
    const operationKind: ConfigureExternalCanisterOperationKind = shouldSoftDelete
      ? { SoftDelete: null }
      : { Delete: null };

    const result = await this.actor.create_request({
      execution_plan: [{ Immediate: null }],
      title: [],
      summary: [],
      operation: {
        ConfigureExternalCanister: {
          canister_id: input.canisterId,
          kind: operationKind,
        },
      },
    });

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok.request;
  }

  async getExternalCanisterStatus(canisterId: Principal): Promise<ExtractOk<CanisterStatusResult>> {
    const result = await this.actor.canister_status({
      canister_id: canisterId,
    });

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok;
  }

  async listAssets(
    { limit, offset }: ListAssetsArgs = {},
    verifiedCall = false,
  ): Promise<ExtractOk<ListAssetsResult>> {
    const actor = verifiedCall ? this.verified_actor : this.actor;
    const result = await actor.list_assets({
      paginate: [
        {
          limit: limit !== undefined ? [limit] : [],
          offset: offset !== undefined ? [BigInt(offset)] : [],
        },
      ],
    });

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok;
  }

  async getExternalCanisterByCanisterId(
    canisterId: Principal,
    verifiedCall = false,
  ): Promise<ExtractOk<GetExternalCanisterResult>> {
    const actor = verifiedCall ? this.verified_actor : this.actor;
    const result = await actor.get_external_canister({
      canister_id: canisterId,
    });

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok;
  }

  async listExternalCanisters(
    { states, labels, canisterIds, limit, offset, sortBy }: ListExternalCanistersArgs = {},
    verifiedCall = false,
  ): Promise<ExtractOk<ListExternalCanistersResult>> {
    const actor = verifiedCall ? this.verified_actor : this.actor;
    const result = await actor.list_external_canisters({
      canister_ids: canisterIds ? [canisterIds] : [],
      labels: labels ? [labels] : [],
      states: states ? [states] : [],
      sort_by: sortBy ? [sortBy] : [],
      paginate: [
        {
          limit: limit ? [limit] : [],
          offset: offset ? [BigInt(offset)] : [],
        },
      ],
    });

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok;
  }

  async addAsset(input: AddAssetOperationInput): Promise<Request> {
    const result = await this.actor.create_request({
      execution_plan: [{ Immediate: null }],
      title: [],
      summary: [],
      operation: { AddAsset: input },
    });

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok.request;
  }

  async fetchExternalCanisterFilters(
    args: {
      with_labels?: boolean;
      with_name?: string;
    } = {},
    verifiedCall = false,
  ): Promise<ExtractOk<GetExternalCanisterFiltersResult>> {
    const actor = verifiedCall ? this.verified_actor : this.actor;
    const result = await actor.get_external_canister_filters({
      with_labels: args.with_labels !== undefined ? [args.with_labels] : [],
      with_name:
        args.with_name !== undefined
          ? [{ prefix: args.with_name.length ? [args.with_name] : [] }]
          : [],
    });

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok;
  }

  async addCanister(input: CreateExternalCanisterOperationInput): Promise<Request> {
    const result = await this.actor.create_request({
      execution_plan: [{ Immediate: null }],
      title: [],
      summary: [],
      operation: { CreateExternalCanister: input },
    });

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok.request;
  }

  async editAsset(input: EditAssetOperationInput): Promise<Request> {
    const result = await this.actor.create_request({
      execution_plan: [{ Immediate: null }],
      title: [],
      summary: [],
      operation: { EditAsset: input },
    });

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok.request;
  }

  async removeAsset(input: RemoveAssetOperationInput): Promise<Request> {
    const result = await this.actor.create_request({
      execution_plan: [{ Immediate: null }],
      title: [],
      summary: [],
      operation: { RemoveAsset: input },
    });

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok.request;
  }

  async getAccount(
    input: GetAccountInput,
    verifiedCall = false,
  ): Promise<ExtractOk<GetAccountResult>> {
    const actor = verifiedCall ? this.verified_actor : this.actor;
    const result = await actor.get_account(input);

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok;
  }

  async getAddressBookEntry(
    input: GetAddressBookEntryInput,
    verifiedCall = false,
  ): Promise<ExtractOk<GetAddressBookEntryResult>> {
    const actor = verifiedCall ? this.verified_actor : this.actor;
    const result = await actor.get_address_book_entry(input);

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok;
  }

  async addAddressBookEntry(input: AddAddressBookEntryOperationInput): Promise<Request> {
    const result = await this.actor.create_request({
      execution_plan: [{ Immediate: null }],
      title: [],
      summary: [],
      operation: { AddAddressBookEntry: input },
    });

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok.request;
  }

  async editAddressBookEntry(input: EditAddressBookEntryOperationInput): Promise<Request> {
    const result = await this.actor.create_request({
      execution_plan: [{ Immediate: null }],
      title: [],
      summary: [],
      operation: { EditAddressBookEntry: input },
    });

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok.request;
  }

  async isHealthy(verifiedCall = false): Promise<boolean> {
    const actor = verifiedCall ? this.verified_actor : this.actor;
    const result = await actor.health_status();

    return variantIs(result, 'Healthy');
  }

  async fetchAccountBalances(input: FetchAccountBalancesInput): Promise<AccountBalance[]> {
    const result = await this.actor.fetch_account_balances(input);

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok.balances;
  }

  async listAccountTransfers(
    input: ListAccountTransfersInput,
    verifiedCall = false,
  ): Promise<TransferListItem[]> {
    const actor = verifiedCall ? this.verified_actor : this.actor;
    const result = await actor.list_account_transfers(input);

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok.transfers;
  }

  async getTransfers(input: GetTransfersInput, verifiedCall = false): Promise<Transfer[]> {
    const actor = verifiedCall ? this.verified_actor : this.actor;
    const result = await actor.get_transfers(input);

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok.transfers;
  }

  async getTransfer(id: UUID, verifiedCall = false): Promise<Transfer> {
    const actor = verifiedCall ? this.verified_actor : this.actor;
    const result = await actor.get_transfers({ transfer_ids: [id] });

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    if (!result.Ok.transfers[0]) {
      throw new Error('Transfer not found');
    }

    return result.Ok.transfers[0];
  }

  async createRequest(input: CreateRequestInput): Promise<Request> {
    const result = await this.actor.create_request(input);

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok.request;
  }

  async addExternalCanister(input: CreateExternalCanisterOperationInput): Promise<Request> {
    const result = await this.actor.create_request({
      execution_plan: [{ Immediate: null }],
      title: [],
      summary: [],
      operation: { CreateExternalCanister: input },
    });

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok.request;
  }

  async callExternalCanister(
    canisterId: Principal,
    call: {
      method: string;
      arg?: Uint8Array | number[];
      attachCycles?: bigint;
      validationMethod?: CanisterMethod;
    },
    opts: { comment?: string } = {},
  ): Promise<Request> {
    const result = await this.actor.create_request({
      execution_plan: [{ Immediate: null }],
      title: [],
      summary: opts.comment ? [opts.comment] : [],
      operation: {
        CallExternalCanister: {
          execution_method: {
            canister_id: canisterId,
            method_name: call.method,
          },
          arg: call.arg !== undefined ? [call.arg] : [],
          execution_method_cycles: call.attachCycles !== undefined ? [call.attachCycles] : [],
          validation_method: call.validationMethod !== undefined ? [call.validationMethod] : [],
        },
      },
    });

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok.request;
  }

  async editExternalCanisterSettings(
    canisterId: Principal,
    input: ConfigureExternalCanisterSettingsInput,
  ): Promise<Request> {
    const result = await this.actor.create_request({
      execution_plan: [{ Immediate: null }],
      title: [],
      summary: [],
      operation: {
        ConfigureExternalCanister: {
          canister_id: canisterId,
          kind: {
            Settings: input,
          },
        },
      },
    });

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok.request;
  }

  async listPermissions(
    input: ListPermissionsInput,
    verifiedCall = false,
  ): Promise<ExtractOk<ListPermissionsResult>> {
    const actor = verifiedCall ? this.verified_actor : this.actor;
    const result = await actor.list_permissions(input);

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok;
  }

  async editPermission(input: EditPermissionOperationInput): Promise<Request> {
    const result = await this.actor.create_request({
      execution_plan: [{ Immediate: null }],
      title: [],
      summary: [],
      operation: { EditPermission: input },
    });

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok.request;
  }

  async editRequestPolicy(input: EditRequestPolicyOperationInput): Promise<Request> {
    const result = await this.actor.create_request({
      execution_plan: [{ Immediate: null }],
      title: [],
      summary: [],
      operation: { EditRequestPolicy: input },
    });

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok.request;
  }

  async addRequestPolicy(input: AddRequestPolicyOperationInput): Promise<Request> {
    const result = await this.actor.create_request({
      execution_plan: [{ Immediate: null }],
      title: [],
      summary: [],
      operation: { AddRequestPolicy: input },
    });

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok.request;
  }

  async createManageSystemInfoRequest(input: ManageSystemInfoOperationInput): Promise<Request> {
    const result = await this.actor.create_request({
      execution_plan: [{ Immediate: null }],
      title: [],
      summary: [],
      operation: { ManageSystemInfo: input },
    });

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok.request;
  }

  async createSetDisasterRecoveryCommitteeRequest(
    input: DisasterRecoveryCommittee,
  ): Promise<Request> {
    const result = await this.actor.create_request({
      execution_plan: [{ Immediate: null }],
      title: [],
      summary: [],
      operation: {
        SetDisasterRecovery: {
          committee: [input],
        },
      },
    });

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok.request;
  }

  async editAccount(input: EditAccountOperationInput): Promise<Request> {
    const result = await this.actor.create_request({
      execution_plan: [{ Immediate: null }],
      title: [],
      summary: [],
      operation: { EditAccount: input },
    });

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok.request;
  }

  async addAccount(input: AddAccountOperationInput): Promise<Request> {
    const result = await this.actor.create_request({
      execution_plan: [{ Immediate: null }],
      title: [],
      summary: [],
      operation: { AddAccount: input },
    });

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok.request;
  }

  async transfer(input: TransferOperationInput, summary?: string): Promise<Request> {
    const result = await this.actor.create_request({
      execution_plan: [{ Immediate: null }],
      title: [],
      summary: summary ? [summary] : [],
      operation: { Transfer: input },
    });

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok.request;
  }

  async systemUpgrade(
    input: SystemUpgradeOperationInput,
    opts: { comment?: string } = {},
  ): Promise<Request> {
    const result = await this.actor.create_request({
      execution_plan: [{ Immediate: null }],
      title: [],
      summary: opts.comment ? [opts.comment] : [],
      operation: { SystemUpgrade: input },
    });

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok.request;
  }

  async listRequestPolicies(
    { limit, offset }: { limit?: number; offset?: number } = {},
    verifiedCall = false,
  ): Promise<ExtractOk<ListRequestPoliciesResult>> {
    const actor = verifiedCall ? this.verified_actor : this.actor;
    const result = await actor.list_request_policies({
      limit: limit ? [limit] : [],
      offset: offset ? [BigInt(offset)] : [],
    });

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok;
  }

  async getRequestPolicy(
    id: UUID,
    verifiedCall = false,
  ): Promise<ExtractOk<GetRequestPolicyResult>> {
    const actor = verifiedCall ? this.verified_actor : this.actor;
    const result = await actor.get_request_policy({ id });

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok;
  }

  async removeRequestPolicy(id: UUID): Promise<Request> {
    const result = await this.actor.create_request({
      execution_plan: [{ Immediate: null }],
      title: [],
      summary: [],
      operation: { RemoveRequestPolicy: { policy_id: id } },
    });

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok.request;
  }

  async removeAddressBookEntry(id: UUID): Promise<Request> {
    const result = await this.actor.create_request({
      execution_plan: [{ Immediate: null }],
      title: [],
      summary: [],
      operation: { RemoveAddressBookEntry: { address_book_entry_id: id } },
    });

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok.request;
  }
}
