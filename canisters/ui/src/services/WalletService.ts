import { Actor, ActorSubclass, HttpAgent } from '@dfinity/agent';
import { Principal } from '@dfinity/principal';
import { variantIs } from '~/core';
import { icAgent } from '~/core/ic-agent';
import { idlFactory } from '~/generated/wallet';
import {
  Account,
  AccountBalance,
  CreateProposalInput,
  FetchAccountBalancesInput,
  GetAccountInput,
  GetProposalInput,
  GetTransfersInput,
  GetUserInput,
  ListAccountTransfersInput,
  ListNotificationsInput,
  ListProposalsInput,
  MarkNotificationsReadInput,
  Notification,
  Proposal,
  Transfer,
  TransferListItem,
  UUID,
  User,
  UserPrivilege,
  VoteOnProposalInput,
  WalletFeatures,
  _SERVICE,
} from '~/generated/wallet/wallet.did';

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

  async features(): Promise<WalletFeatures> {
    const result = await this.actor.features();

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok.features;
  }

  async listNotifications(input: ListNotificationsInput): Promise<Notification[]> {
    const result = await this.actor.list_notifications(input);

    if (variantIs(result, 'Err')) {
      throw result.Err;
    }

    return result.Ok.notifications;
  }

  async listProposals(input: ListProposalsInput): Promise<Proposal[]> {
    const result = await this.actor.list_proposals(input);

    if (variantIs(result, 'Err')) {
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
