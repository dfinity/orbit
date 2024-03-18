import { Actor, ActorSubclass, HttpAgent } from '@dfinity/agent';
import { Principal } from '@dfinity/principal';
import { idlFactory } from '~/generated/wallet';
import {
  AccountBalance,
  AddAccountOperationInput,
  AddAddressBookEntryOperationInput,
  AddProposalPolicyOperationInput,
  AddUserGroupOperationInput,
  AddUserOperationInput,
  ChangeCanisterOperationInput,
  Config,
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
} from '~/generated/wallet/wallet.did';
import { ExtractOk } from '~/types/helper.types';
import {
  GetNextVotableProposalArgs,
  ListAccountsArgs,
  ListAddressBookEntriesArgs,
  ListProposalsArgs,
} from '~/types/wallet.types';
import { variantIs } from '~/utils/helper.utils';

export class WalletService {
  private actor: ActorSubclass<_SERVICE>;

  public static ERR_USER_IDENTITY_NOT_FOUND = 'NOT_FOUND_USER_IDENTITY';
  public static ERR_USER_NOT_FOUND = 'NOT_FOUND';

  constructor(
    private agent: HttpAgent,
    walletId: Principal = Principal.anonymous(),
  ) {
    this.actor = Actor.createActor<_SERVICE>(idlFactory, {
      agent: this.agent,
      canisterId: walletId,
    });
  }

  withWalletId(walletId: Principal): WalletService {
    this.actor = Actor.createActor<_SERVICE>(idlFactory, {
      agent: this.agent,
      canisterId: walletId,
    });

    return this;
  }

  async getUser(input: GetUserInput): Promise<ExtractOk<GetUserResult>> {
    const result = await this.actor.get_user(input);
    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok;
  }

  async getAccessPolicy(input: GetAccessPolicyInput): Promise<ExtractOk<GetAccessPolicyResult>> {
    const result = await this.actor.get_access_policy(input);
    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok;
  }

  async getUserGroup(input: GetUserGroupInput): Promise<ExtractOk<GetUserGroupResult>> {
    const result = await this.actor.get_user_group(input);
    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok;
  }

  async myUser(): Promise<{ me: User; privileges: UserPrivilege[] } | null> {
    const result = await this.actor.me();
    if (variantIs(result, 'Err')) {
      if (result.Err.code === WalletService.ERR_USER_NOT_FOUND) {
        return null;
      }

      throw result.Err;
    }

    return {
      me: result.Ok.me,
      privileges: result.Ok.privileges,
    };
  }

  async listUserGroups({
    limit,
    offset,
    searchTerm,
  }: {
    limit?: number;
    offset?: number;
    searchTerm?: string;
  } = {}): Promise<ExtractOk<ListUserGroupsResult>> {
    const result = await this.actor.list_user_groups({
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

  async listUsers({
    limit,
    offset,
    searchTerm,
    statuses,
  }: {
    limit?: number;
    offset?: number;
    searchTerm?: string;
    statuses?: UserStatus[];
  } = {}): Promise<ExtractOk<ListUsersResult>> {
    const result = await this.actor.list_users({
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

  async config(): Promise<Config> {
    const result = await this.actor.config();

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok.config;
  }

  async listNotifications(input: ListNotificationsInput): Promise<Notification[]> {
    const result = await this.actor.list_notifications(input);

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok.notifications;
  }

  async listProposals({
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
  }: ListProposalsArgs = {}): Promise<ExtractOk<ListProposalsResult>> {
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

    const result = await this.actor.list_proposals({
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
    });

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok;
  }

  async getNextVotableProposal({
    types,
    excludedProposalIds,
  }: GetNextVotableProposalArgs = {}): Promise<ExtractOk<GetNextVotableProposalResponse>> {
    const result = await this.actor.get_next_votable_proposal({
      operation_types: types ? [types] : [],
      excluded_proposal_ids: excludedProposalIds ?? [],
    });

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok;
  }

  async listUnreadNotifications(from_dt?: Date, last_id?: UUID): Promise<Notification[]> {
    const notifications = await this.listNotifications({
      notification_type: [],
      status: [{ Sent: null }],
      from_dt: from_dt ? [from_dt.toISOString()] : [],
      to_dt: [],
    });

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

  async getProposal(input: GetProposalInput): Promise<ExtractOk<GetProposalResult>> {
    const result = await this.actor.get_proposal(input);

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok;
  }

  async listAccounts({ limit, offset, searchTerm }: ListAccountsArgs = {}): Promise<
    ExtractOk<ListAccountsResult>
  > {
    const result = await this.actor.list_accounts({
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

  async listAddressBook({
    limit,
    offset,
    blockchain,
    standard,
    ids,
    addresses,
  }: ListAddressBookEntriesArgs = {}): Promise<ExtractOk<ListAddressBookEntriesResult>> {
    const result = await this.actor.list_address_book_entries({
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

  async getAccount(input: GetAccountInput): Promise<ExtractOk<GetAccountResult>> {
    const result = await this.actor.get_account(input);

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok;
  }

  async getAddressBookEntry(
    input: GetAddressBookEntryInput,
  ): Promise<ExtractOk<GetAddressBookEntryResult>> {
    const result = await this.actor.get_address_book_entry(input);

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

  async isHealthy(): Promise<boolean> {
    const result = await this.actor.health_status();

    return variantIs(result, 'Healthy');
  }

  async fetchAccountBalances(input: FetchAccountBalancesInput): Promise<AccountBalance[]> {
    const result = await this.actor.fetch_account_balances(input);

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok.balances;
  }

  async listAccountTransfers(input: ListAccountTransfersInput): Promise<TransferListItem[]> {
    const result = await this.actor.list_account_transfers(input);

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok.transfers;
  }

  async getTransfers(input: GetTransfersInput): Promise<Transfer[]> {
    const result = await this.actor.get_transfers(input);

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok.transfers;
  }

  async getTransfer(id: UUID): Promise<Transfer> {
    const result = await this.actor.get_transfers({ transfer_ids: [id] });

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
  ): Promise<ExtractOk<ListAccessPoliciesResult>> {
    const result = await this.actor.list_access_policies(input);

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

  async listProposalPolicies({ limit, offset }: { limit?: number; offset?: number } = {}): Promise<
    ExtractOk<ListProposalPoliciesResult>
  > {
    const result = await this.actor.list_proposal_policies({
      limit: limit ? [limit] : [],
      offset: offset ? [BigInt(offset)] : [],
    });

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok;
  }

  async getProposalPolicy(id: UUID): Promise<ExtractOk<GetProposalPolicyResult>> {
    const result = await this.actor.get_proposal_policy({ id });

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
