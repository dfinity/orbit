import { Actor, ActorSubclass, HttpAgent } from '@dfinity/agent';
import { Principal } from '@dfinity/principal';
import { idlFactory } from '~/generated/station';
import {
  AccountBalance,
  AddAccountOperationInput,
  AddAddressBookEntryOperationInput,
  AddRequestPolicyOperationInput,
  AddUserGroupOperationInput,
  AddUserOperationInput,
  Capabilities,
  ChangeCanisterOperationInput,
  CreateRequestInput,
  EditAccountOperationInput,
  EditAddressBookEntryOperationInput,
  EditPermissionOperationInput,
  EditRequestPolicyOperationInput,
  EditUserGroupOperationInput,
  EditUserOperationInput,
  FetchAccountBalancesInput,
  GetAccountInput,
  GetAccountResult,
  GetAddressBookEntryInput,
  GetAddressBookEntryResult,
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
  RemoveUserGroupOperationInput,
  Request,
  SubmitRequestApprovalInput,
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
    }: {
      limit?: number;
      offset?: number;
      searchTerm?: string;
      statuses?: UserStatus[];
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

  async listAddressBook(
    { limit, offset, blockchain, standard, ids, addresses }: ListAddressBookEntriesArgs = {},
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
      address_chain:
        blockchain && standard
          ? [
              {
                blockchain: blockchain,
                standard: standard,
              },
            ]
          : [],
      addresses: addresses ? [addresses] : [],
      ids: ids ? [ids] : [],
    });

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok;
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

  async changeCanister(input: ChangeCanisterOperationInput): Promise<Request> {
    const result = await this.actor.create_request({
      execution_plan: [{ Immediate: null }],
      title: [],
      summary: [],
      operation: { ChangeCanister: input },
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
