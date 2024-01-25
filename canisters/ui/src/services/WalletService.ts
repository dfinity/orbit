import { Actor, ActorSubclass, HttpAgent } from '@dfinity/agent';
import { Principal } from '@dfinity/principal';
import { variantIs } from '~/core';
import { icAgent } from '~/core/ic-agent';
import { idlFactory } from '~/generated/wallet';
import {
  Account,
  AccountBalance,
  AddUserGroupOperationInput,
  AddUserOperationInput,
  Config,
  CreateProposalInput,
  EditUserGroupOperationInput,
  EditUserOperationInput,
  FetchAccountBalancesInput,
  GetAccountInput,
  GetProposalInput,
  GetTransfersInput,
  GetUserInput,
  ListAccountTransfersInput,
  ListNotificationsInput,
  ListProposalsInput,
  ListProposalsResult,
  ListUsersResult,
  MarkNotificationsReadInput,
  Notification,
  PaginationInput,
  Proposal,
  RemoveUserGroupOperationInput,
  Transfer,
  TransferListItem,
  UUID,
  User,
  UserGroup,
  UserPrivilege,
  VoteOnProposalInput,
  _SERVICE,
} from '~/generated/wallet/wallet.did';
import { ExtractOk, ListProposalsArgs } from '~/types';

export class WalletService {
  private actor: ActorSubclass<_SERVICE>;

  public static ERR_USER_IDENTITY_NOT_FOUND = 'NOT_FOUND_USER_IDENTITY';
  public static ERR_USER_NOT_FOUND = 'NOT_FOUND';

  constructor(
    private agent: HttpAgent = icAgent.get(),
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

  async getUser(input: GetUserInput): Promise<User> {
    const result = await this.actor.get_user(input);
    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok.user;
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

  async listUserGroups(): Promise<UserGroup[]> {
    const result = await this.actor.list_user_groups();
    if ('Err' in result) {
      throw result.Err;
    }

    return result.Ok.user_groups;
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

  async listUsers({ limit, offset }: { limit?: number; offset?: number } = {}): Promise<
    ExtractOk<ListUsersResult>
  > {
    const result = await this.actor.list_users({
      limit: limit ? [limit] : [],
      offset: offset ? [BigInt(offset)] : [],
    });

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return {
      users: result.Ok.users,
      next_offset: result.Ok.next_offset,
      total: result.Ok.total,
    };
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

  async getProposal(input: GetProposalInput): Promise<Proposal> {
    const result = await this.actor.get_proposal(input);

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok.proposal;
  }

  async listAccounts(): Promise<Account[]> {
    const result = await this.actor.list_accounts();

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok.accounts;
  }

  async getAccount(input: GetAccountInput): Promise<Account> {
    const result = await this.actor.get_account(input);

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok.account;
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

  async createProposal(input: CreateProposalInput): Promise<Proposal> {
    const result = await this.actor.create_proposal(input);

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok.proposal;
  }
}
