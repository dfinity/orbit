import { Actor, ActorSubclass, HttpAgent } from '@dfinity/agent';
import { Principal } from '@dfinity/principal';
import { icAgent } from '~/core/IcAgent';
import { idlFactory } from '~/generated/bank';
import {
  User,
  Error as ApiError,
  BankFeatures,
  ConfirmUserIdentityInput,
  CreateAccountInput,
  EditUserInput,
  VoteOnProposalInput,
  FetchAccountBalancesInput,
  GetUserInput,
  GetProposalInput,
  GetTransferInput,
  GetTransfersInput,
  GetAccountInput,
  Notification,
  ListAccountProposalsInput,
  ListAccountTransfersInput,
  Proposal,
  RegisterUserInput,
  Transfer,
  TransferInput,
  TransferListItem,
  Account,
  AccountBalance,
  _SERVICE,
  ListNotificationsInput,
  NotificationId,
  MarkNotificationsReadInput,
} from '~/generated/bank/bank.did';
import { Maybe } from '~/types';

export class BankService {
  private actor: ActorSubclass<_SERVICE>;

  public static ERR_USER_IDENTITY_NOT_FOUND = 'NOT_FOUND_USER_IDENTITY';

  constructor(
    private agent: HttpAgent = icAgent.get(),
    bankId: Principal = Principal.anonymous(),
  ) {
    this.actor = Actor.createActor<_SERVICE>(idlFactory, {
      agent: this.agent,
      canisterId: bankId,
    });
  }

  withBankId(bankId: Principal): BankService {
    this.actor = Actor.createActor<_SERVICE>(idlFactory, {
      agent: this.agent,
      canisterId: bankId,
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

  async myUser(): Promise<Maybe<User>> {
    return this.getUser({ user_id: [] }).catch((err: ApiError) => {
      if (err.code === BankService.ERR_USER_IDENTITY_NOT_FOUND) {
        return null;
      }

      throw err;
    });
  }

  async register(input: RegisterUserInput): Promise<User> {
    const result = await this.actor.register_user(input);

    if ('Err' in result) {
      throw result.Err;
    }

    return result.Ok.user;
  }

  async editUser(input: EditUserInput): Promise<User> {
    const result = await this.actor.edit_user(input);

    if ('Err' in result) {
      throw result.Err;
    }

    return result.Ok.user;
  }

  async features(): Promise<BankFeatures> {
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

  async listAccountProposals(input: ListAccountProposalsInput): Promise<Proposal[]> {
    const result = await this.actor.list_account_proposals(input);

    if ('Err' in result) {
      throw result.Err;
    }

    return result.Ok.proposals;
  }

  async listUnreadNotifications(from_dt?: Date, last_id?: NotificationId): Promise<Notification[]> {
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

  async createAccount(input: CreateAccountInput): Promise<Account> {
    const result = await this.actor.create_account(input);

    if ('Err' in result) {
      throw result.Err;
    }

    return result.Ok.account;
  }

  async listAccountTransfers(input: ListAccountTransfersInput): Promise<TransferListItem[]> {
    const result = await this.actor.list_account_transfers(input);

    if ('Err' in result) {
      throw result.Err;
    }

    return result.Ok.transfers;
  }

  async getTransfer(input: GetTransferInput): Promise<Transfer> {
    const result = await this.actor.get_transfer(input);

    if ('Err' in result) {
      throw result.Err;
    }

    return result.Ok.transfer;
  }

  async getTransfers(input: GetTransfersInput): Promise<Transfer[]> {
    const result = await this.actor.get_transfers(input);

    if ('Err' in result) {
      throw result.Err;
    }

    return result.Ok.transfers;
  }

  async createTransfer(input: TransferInput): Promise<Transfer> {
    const result = await this.actor.transfer(input);

    if ('Err' in result) {
      throw result.Err;
    }

    return result.Ok.transfer;
  }

  async confirmUserIdentity(input: ConfirmUserIdentityInput): Promise<User> {
    const result = await this.actor.confirm_user_identity(input);

    if ('Err' in result) {
      throw result.Err;
    }

    return result.Ok.user;
  }
}
