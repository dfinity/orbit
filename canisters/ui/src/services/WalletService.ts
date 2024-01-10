import { Actor, ActorSubclass, HttpAgent } from '@dfinity/agent';
import { Principal } from '@dfinity/principal';
import { icAgent } from '~/core/ic-agent';
import { idlFactory } from '~/generated/wallet';
import {
  User,
  WalletFeatures,
  VoteOnProposalInput,
  FetchAccountBalancesInput,
  GetUserInput,
  GetProposalInput,
  GetTransfersInput,
  GetAccountInput,
  Notification,
  ListAccountTransfersInput,
  Proposal,
  Transfer,
  CreateProposalInput,
  TransferListItem,
  Account,
  AccountBalance,
  _SERVICE,
  ListNotificationsInput,
  MarkNotificationsReadInput,
  ListProposalsInput,
  UUID,
} from '~/generated/wallet/wallet.did';
import { AuthenticatedUser } from '~/types';

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
    if ('Err' in result) {
      throw result.Err;
    }

    return result.Ok.user;
  }

  async myUser(): Promise<AuthenticatedUser | null> {
    const result = await this.actor.me();
    if ('Err' in result) {
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

  async features(): Promise<WalletFeatures> {
    const result = await this.actor.features();

    if ('Err' in result) {
      throw result.Err;
    }

    return result.Ok.features;
  }

  async listNotifications(input: ListNotificationsInput): Promise<Notification[]> {
    const result = await this.actor.list_notifications(input);

    if ('Err' in result) {
      throw result.Err;
    }

    return result.Ok.notifications;
  }

  async listProposals(input: ListProposalsInput): Promise<Proposal[]> {
    const result = await this.actor.list_proposals(input);

    if ('Err' in result) {
      throw result.Err;
    }

    return result.Ok.proposals;
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

    if ('Err' in result) {
      throw result.Err;
    }

    return result.Ok.proposal;
  }

  async getProposal(input: GetProposalInput): Promise<Proposal> {
    const result = await this.actor.get_proposal(input);

    if ('Err' in result) {
      throw result.Err;
    }

    return result.Ok.proposal;
  }

  async listAccounts(): Promise<Account[]> {
    const result = await this.actor.list_accounts();

    if ('Err' in result) {
      throw result.Err;
    }

    return result.Ok.accounts;
  }

  async getAccount(input: GetAccountInput): Promise<Account> {
    const result = await this.actor.get_account(input);

    if ('Err' in result) {
      throw result.Err;
    }

    return result.Ok.account;
  }

  async fetchAccountBalances(input: FetchAccountBalancesInput): Promise<AccountBalance[]> {
    const result = await this.actor.fetch_account_balances(input);

    if ('Err' in result) {
      throw result.Err;
    }

    return result.Ok.balances;
  }

  async listAccountTransfers(input: ListAccountTransfersInput): Promise<TransferListItem[]> {
    const result = await this.actor.list_account_transfers(input);

    if ('Err' in result) {
      throw result.Err;
    }

    return result.Ok.transfers;
  }

  async getTransfers(input: GetTransfersInput): Promise<Transfer[]> {
    const result = await this.actor.get_transfers(input);

    if ('Err' in result) {
      throw result.Err;
    }

    return result.Ok.transfers;
  }

  async createProposal(input: CreateProposalInput): Promise<Proposal> {
    const result = await this.actor.create_proposal(input);

    if ('Err' in result) {
      throw result.Err;
    }

    return result.Ok.proposal;
  }
}
