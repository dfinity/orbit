import { Actor, ActorSubclass, HttpAgent } from '@dfinity/agent';
import { Principal } from '@dfinity/principal';
import { icAgent } from '~/core/IcAgent';
import {
  Account,
  Error as ApiError,
  BankFeatures,
  ConfirmAccountInput,
  CreateWalletInput,
  EditAccountInput,
  EditOperationInput,
  GetAccountInput,
  GetOperationInput,
  GetTransferInput,
  GetWalletBalanceInput,
  ListOperationsInput,
  ListWalletTransfersInput,
  Operation,
  OperationListItem,
  RegisterAccountInput,
  Transfer,
  TransferInput,
  TransferListItem,
  Wallet,
  WalletBalance,
  WalletListItem,
  _SERVICE,
} from '~/generated/bank/bank.did';
import { idlFactory } from '~/generated/control-panel';
import { Maybe } from '~/types';

export class BankService {
  private actor: ActorSubclass<_SERVICE>;

  public static ACCOUNT_NOT_FOUND_ERROR = 'ACCOUNT_NOT_FOUND';

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

  async getAccount(input: GetAccountInput): Promise<Account> {
    const result = await this.actor.get_account(input);
    if ('Err' in result) {
      throw result.Err;
    }

    return result.Ok.account;
  }

  async myAccount(): Promise<Maybe<Account>> {
    return this.getAccount({ account_id: [] }).catch((err: ApiError) => {
      if (err.code === BankService.ACCOUNT_NOT_FOUND_ERROR) {
        return null;
      }

      throw err;
    });
  }

  async register(input: RegisterAccountInput): Promise<Account> {
    const result = await this.actor.register_account(input);

    if ('Err' in result) {
      throw result.Err;
    }

    return result.Ok.account;
  }

  async editAccount(input: EditAccountInput): Promise<Account> {
    const result = await this.actor.edit_account(input);

    if ('Err' in result) {
      throw result.Err;
    }

    return result.Ok.account;
  }

  async features(): Promise<BankFeatures> {
    const result = await this.actor.features();

    if ('Err' in result) {
      throw result.Err;
    }

    return result.Ok.features;
  }

  async listOperations(input: ListOperationsInput): Promise<OperationListItem[]> {
    const result = await this.actor.list_operations(input);

    if ('Err' in result) {
      throw result.Err;
    }

    return result.Ok.operations;
  }

  async listUnreadOperations(): Promise<OperationListItem[]> {
    return this.listOperations({ read: [false], code: [], status: [] });
  }

  async editOperation(input: EditOperationInput): Promise<Operation> {
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

  async listWallets(): Promise<WalletListItem[]> {
    const result = await this.actor.list_wallets();

    if ('Err' in result) {
      throw result.Err;
    }

    return result.Ok.wallets;
  }

  async walletBalance(input: GetWalletBalanceInput): Promise<WalletBalance> {
    const result = await this.actor.get_wallet_balance(input);

    if ('Err' in result) {
      throw result.Err;
    }

    return result.Ok.balance;
  }

  async createWallet(input: CreateWalletInput): Promise<Wallet> {
    const result = await this.actor.create_wallet(input);

    if ('Err' in result) {
      throw result.Err;
    }

    return result.Ok.wallet;
  }

  async listWalletTransfers(input: ListWalletTransfersInput): Promise<TransferListItem[]> {
    const result = await this.actor.list_wallet_transfers(input);

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

  async createTransfer(input: TransferInput): Promise<Transfer> {
    const result = await this.actor.transfer(input);

    if ('Err' in result) {
      throw result.Err;
    }

    return result.Ok.transfer;
  }

  async confirmAccount(input: ConfirmAccountInput): Promise<Account> {
    const result = await this.actor.confirm_account(input);

    if ('Err' in result) {
      throw result.Err;
    }

    return result.Ok.account;
  }
}
