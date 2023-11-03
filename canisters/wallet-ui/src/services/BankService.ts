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
  EditOperationInput,
  FetchAccountBalancesInput,
  GetUserInput,
  GetOperationInput,
  GetTransferInput,
  GetTransfersInput,
  GetAccountInput,
  ListOperationsInput,
  ListAccountOperationsInput,
  ListAccountTransfersInput,
  Operation,
  OperationId,
  RegisterUserInput,
  Transfer,
  TransferInput,
  TransferListItem,
  Account,
  AccountBalance,
  _SERVICE,
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

  async listOperations(input: ListOperationsInput): Promise<Operation[]> {
    const result = await this.actor.list_operations(input);

    if ('Err' in result) {
      throw result.Err;
    }

    return result.Ok.operations;
  }

  async listAccountOperations(input: ListAccountOperationsInput): Promise<Operation[]> {
    const result = await this.actor.list_account_operations(input);

    if ('Err' in result) {
      throw result.Err;
    }

    return result.Ok.operations;
  }

  async listUnreadPendingOperations(from_dt?: Date, last_id?: OperationId): Promise<Operation[]> {
    const operations = await this.listOperations({
      read: [false],
      code: [],
      status: [{ Pending: null }],
      from_dt: from_dt ? [from_dt.toISOString()] : [],
      to_dt: [],
    });

    return operations.filter(operation => operation.id !== last_id);
  }

  async submitOperationDecision(input: EditOperationInput): Promise<Operation> {
    const result = await this.actor.edit_operation(input);

    if ('Err' in result) {
      throw result.Err;
    }

    return result.Ok.operation;
  }

  async getOperation(input: GetOperationInput): Promise<Operation> {
    const result = await this.actor.get_operation(input);

    if ('Err' in result) {
      throw result.Err;
    }

    return result.Ok.operation;
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
