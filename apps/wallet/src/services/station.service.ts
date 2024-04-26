import { Actor, ActorSubclass, HttpAgent } from '@dfinity/agent';
import { Principal } from '@dfinity/principal';
import { idlFactory } from '~/generated/station';
import {
  AccountBalance,
  AddAccountOperationInput,
  AddAddressBookEntryOperationInput,
  AddProposalPolicyOperationInput,
  AddUserGroupOperationInput,
  AddUserOperationInput,
  Capabilities,
  ChangeCanisterOperationInput,
  CreateProposalInput,
  EditAccessPolicyOperationInput,
  EditAccountOperationInput,
  EditAddressBookEntryOperationInput,
  EditProposalPolicyOperationInput,
  EditUserGroupOperationInput,
  EditUserOperationInput,
  FetchAccountBalancesInput,
  GetAccessPolicyInput,
  GetAccessPolicyResult,
  GetAccountInput,
  GetAccountResult,
  GetAddressBookEntryInput,
  GetAddressBookEntryResult,
  GetNextVotableProposalResponse,
  GetProposalInput,
  GetProposalPolicyResult,
  GetProposalResult,
  GetTransfersInput,
  GetUserGroupInput,
  GetUserGroupResult,
  GetUserInput,
  GetUserResult,
  ListAccessPoliciesInput,
  ListAccessPoliciesResult,
  ListAccountTransfersInput,
  ListAccountsResult,
  ListAddressBookEntriesResult,
  ListNotificationsInput,
  ListProposalPoliciesResult,
  ListProposalsInput,
  ListProposalsResult,
  ListUserGroupsResult,
  ListUsersResult,
  MarkNotificationsReadInput,
  Notification,
  PaginationInput,
  Proposal,
  RemoveUserGroupOperationInput,
  Transfer,
  TransferListItem,
  TransferOperationInput,
  UUID,
  User,
  UserPrivilege,
  UserStatus,
  VoteOnProposalInput,
  _SERVICE,
} from '~/generated/station/station.did';
import { ExtractOk } from '~/types/helper.types';
import {
  GetNextVotableProposalArgs,
  ListAccountsArgs,
  ListAddressBookEntriesArgs,
  ListProposalsArgs,
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
    stationId: Principal = Principal.anonymous(),
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

  withStationId(stationId: Principal): StationService {
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

  async getAccessPolicy(
    input: GetAccessPolicyInput,
    verifiedCall = false,
  ): Promise<ExtractOk<GetAccessPolicyResult>> {
    const actor = verifiedCall ? this.verified_actor : this.actor;
    const result = await actor.get_access_policy(input);

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok;
  }

  async getAccountAccessPolicies(
    accountId: UUID,
    verifiedCall = false,
  ): Promise<ExtractOk<ListAccessPoliciesResult>> {
    const actor = verifiedCall ? this.verified_actor : this.actor;

    const result = await actor.list_access_policies({
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

  async removeUserGroup(input: RemoveUserGroupOperationInput): Promise<Proposal> {
    const result = await this.actor.create_proposal({
      execution_plan: [{ Immediate: null }],
      title: [],
      summary: [],
      operation: { RemoveUserGroup: input },
    });

    if ('Err' in result) {
      throw result.Err;
    }

    return result.Ok.proposal;
  }

  async addUserGroup(input: AddUserGroupOperationInput): Promise<Proposal> {
    const result = await this.actor.create_proposal({
      execution_plan: [{ Immediate: null }],
      title: [],
      summary: [],
      operation: { AddUserGroup: input },
    });

    if ('Err' in result) {
      throw result.Err;
    }

    return result.Ok.proposal;
  }

  async editUserGroup(input: EditUserGroupOperationInput): Promise<Proposal> {
    const result = await this.actor.create_proposal({
      execution_plan: [{ Immediate: null }],
      title: [],
      summary: [],
      operation: { EditUserGroup: input },
    });

    if ('Err' in result) {
      throw result.Err;
    }

    return result.Ok.proposal;
  }

  async addUser(input: AddUserOperationInput): Promise<Proposal> {
    const result = await this.actor.create_proposal({
      execution_plan: [{ Immediate: null }],
      title: [],
      summary: [],
      operation: { AddUser: input },
    });

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok.proposal;
  }

  async editUser(input: EditUserOperationInput): Promise<Proposal> {
    const result = await this.actor.create_proposal({
      execution_plan: [{ Immediate: null }],
      title: [],
      summary: [],
      operation: { EditUser: input },
    });

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok.proposal;
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

  async listProposals(
    {
      created_dt,
      expiration_dt,
      limit,
      offset,
      proposerIds,
      statuses,
      types,
      voterIds,
      sortBy,
      onlyVotable,
    }: ListProposalsArgs = {},
    verifiedCall = false,
  ): Promise<ExtractOk<ListProposalsResult>> {
    const actor = verifiedCall ? this.verified_actor : this.actor;
    const paginate: PaginationInput = {
      limit: limit ? [limit] : [],
      offset: offset ? [BigInt(offset)] : [],
    };

    let sortingCriteria: ListProposalsInput['sort_by'] | [] = [];
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

    const result = await actor.list_proposals({
      statuses: statuses ? [statuses] : [],
      created_from_dt: created_dt?.fromDt ? [created_dt.fromDt.toISOString()] : [],
      created_to_dt: created_dt?.toDt ? [created_dt.toDt.toISOString()] : [],
      expiration_from_dt: expiration_dt?.fromDt ? [expiration_dt.fromDt.toISOString()] : [],
      expiration_to_dt: expiration_dt?.toDt ? [expiration_dt.toDt.toISOString()] : [],
      operation_types: types ? [types] : [],
      proposer_ids: proposerIds ? [proposerIds] : [],
      voter_ids: voterIds ? [voterIds] : [],
      paginate: [paginate],
      sort_by: sortingCriteria,
      only_votable: !!onlyVotable,
      with_evaluation_results: false,
    });

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok;
  }

  async getNextVotableProposal(
    { types, excludedProposalIds }: GetNextVotableProposalArgs = {},
    verifiedCall = false,
  ): Promise<ExtractOk<GetNextVotableProposalResponse>> {
    const actor = verifiedCall ? this.verified_actor : this.actor;
    const result = await actor.get_next_votable_proposal({
      operation_types: types ? [types] : [],
      excluded_proposal_ids: excludedProposalIds ?? [],
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

  async voteOnProposal(input: VoteOnProposalInput): Promise<Proposal> {
    const result = await this.actor.vote_on_proposal(input);

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok.proposal;
  }

  async getProposal(
    input: GetProposalInput,
    verifiedCall = false,
  ): Promise<ExtractOk<GetProposalResult>> {
    const actor = verifiedCall ? this.verified_actor : this.actor;
    const result = await actor.get_proposal(input);

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

  async addAddressBookEntry(input: AddAddressBookEntryOperationInput): Promise<Proposal> {
    const result = await this.actor.create_proposal({
      execution_plan: [{ Immediate: null }],
      title: [],
      summary: [],
      operation: { AddAddressBookEntry: input },
    });

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok.proposal;
  }

  async editAddressBookEntry(input: EditAddressBookEntryOperationInput): Promise<Proposal> {
    const result = await this.actor.create_proposal({
      execution_plan: [{ Immediate: null }],
      title: [],
      summary: [],
      operation: { EditAddressBookEntry: input },
    });

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok.proposal;
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

  async createProposal(input: CreateProposalInput): Promise<Proposal> {
    const result = await this.actor.create_proposal(input);

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok.proposal;
  }

  async listAccessPolicies(
    input: ListAccessPoliciesInput,
    verifiedCall = false,
  ): Promise<ExtractOk<ListAccessPoliciesResult>> {
    const actor = verifiedCall ? this.verified_actor : this.actor;
    const result = await actor.list_access_policies(input);

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok;
  }

  async editAccessPolicy(input: EditAccessPolicyOperationInput): Promise<Proposal> {
    const result = await this.actor.create_proposal({
      execution_plan: [{ Immediate: null }],
      title: [],
      summary: [],
      operation: { EditAccessPolicy: input },
    });

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok.proposal;
  }

  async editProposalPolicy(input: EditProposalPolicyOperationInput): Promise<Proposal> {
    const result = await this.actor.create_proposal({
      execution_plan: [{ Immediate: null }],
      title: [],
      summary: [],
      operation: { EditProposalPolicy: input },
    });

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok.proposal;
  }

  async addProposalPolicy(input: AddProposalPolicyOperationInput): Promise<Proposal> {
    const result = await this.actor.create_proposal({
      execution_plan: [{ Immediate: null }],
      title: [],
      summary: [],
      operation: { AddProposalPolicy: input },
    });

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok.proposal;
  }

  async editAccount(input: EditAccountOperationInput): Promise<Proposal> {
    const result = await this.actor.create_proposal({
      execution_plan: [{ Immediate: null }],
      title: [],
      summary: [],
      operation: { EditAccount: input },
    });

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok.proposal;
  }

  async addAccount(input: AddAccountOperationInput): Promise<Proposal> {
    const result = await this.actor.create_proposal({
      execution_plan: [{ Immediate: null }],
      title: [],
      summary: [],
      operation: { AddAccount: input },
    });

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok.proposal;
  }

  async transfer(input: TransferOperationInput, summary?: string): Promise<Proposal> {
    const result = await this.actor.create_proposal({
      execution_plan: [{ Immediate: null }],
      title: [],
      summary: summary ? [summary] : [],
      operation: { Transfer: input },
    });

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok.proposal;
  }

  async changeCanister(input: ChangeCanisterOperationInput): Promise<Proposal> {
    const result = await this.actor.create_proposal({
      execution_plan: [{ Immediate: null }],
      title: [],
      summary: [],
      operation: { ChangeCanister: input },
    });

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok.proposal;
  }

  async listProposalPolicies(
    { limit, offset }: { limit?: number; offset?: number } = {},
    verifiedCall = false,
  ): Promise<ExtractOk<ListProposalPoliciesResult>> {
    const actor = verifiedCall ? this.verified_actor : this.actor;
    const result = await actor.list_proposal_policies({
      limit: limit ? [limit] : [],
      offset: offset ? [BigInt(offset)] : [],
    });

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok;
  }

  async getProposalPolicy(
    id: UUID,
    verifiedCall = false,
  ): Promise<ExtractOk<GetProposalPolicyResult>> {
    const actor = verifiedCall ? this.verified_actor : this.actor;
    const result = await actor.get_proposal_policy({ id });

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok;
  }

  async removeProposalPolicy(id: UUID): Promise<Proposal> {
    const result = await this.actor.create_proposal({
      execution_plan: [{ Immediate: null }],
      title: [],
      summary: [],
      operation: { RemoveProposalPolicy: { policy_id: id } },
    });

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok.proposal;
  }

  async removeAddressBookEntry(id: UUID): Promise<Proposal> {
    const result = await this.actor.create_proposal({
      execution_plan: [{ Immediate: null }],
      title: [],
      summary: [],
      operation: { RemoveAddressBookEntry: { address_book_entry_id: id } },
    });

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok.proposal;
  }
}
