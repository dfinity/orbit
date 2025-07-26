import { Actor, ActorSubclass, HttpAgent } from '@icp-sdk/core/agent';
import { icAgent } from '~/core/ic-agent.core';
import {
  AccountIncomingTransfer,
  ChainApi,
  ChainApiCapability,
  FetchTransfersInput,
} from '~/types/chain.types';
import { nanoToJsDate } from '~/utils/date.utils';
import { decodeIcrcAccount, encodeIcrcAccount } from '@dfinity/ledger-icrc';
import { Account } from '~/generated/icp_index/icp_index.did';
import { idlFactory as Icrc1IndexIdlFactory } from '~/generated/icrc1_index';
import { idlFactory as Icrc1LedgerIdlFactory } from '~/generated/icrc1_ledger';
import { _SERVICE as Icrc1IndexService } from '~/generated/icrc1_index/icrc1_index_canister.did';
import { _SERVICE as Icrc1LedgerService } from '~/generated/icrc1_ledger/icrc1_ledger_canister.did';

export class ICRC1Api implements ChainApi {
  private indexActor: ActorSubclass<Icrc1IndexService> | null = null;
  private ledgerActor: ActorSubclass<Icrc1LedgerService>;
  static PAGE_SIZE = BigInt(100);

  private account: Account;

  constructor(
    address: string,
    private readonly ledgerCanisterId: string,
    private readonly indexCanisterId: string | undefined,
    agent: HttpAgent = icAgent.get(),
  ) {
    const icrc1Account = decodeIcrcAccount(address);

    this.account = {
      owner: icrc1Account.owner,
      subaccount: icrc1Account.subaccount ? [icrc1Account.subaccount] : [],
    };

    if (this.indexCanisterId) {
      this.indexActor = Actor.createActor<Icrc1IndexService>(Icrc1IndexIdlFactory, {
        agent,
        canisterId: this.indexCanisterId,
      });
    }

    this.ledgerActor = Actor.createActor<Icrc1LedgerService>(Icrc1LedgerIdlFactory, {
      agent,
      canisterId: this.ledgerCanisterId,
    });
  }

  static isValidAddress(address: string): boolean {
    try {
      decodeIcrcAccount(address);
      return true;
    } catch {
      return false;
    }
  }
  isValidAddress(address: string): boolean {
    return ICRC1Api.isValidAddress(address);
  }

  async fetchBalance(): Promise<bigint> {
    return await this.ledgerActor.icrc1_balance_of(this.account);
  }

  async fetchTransfers(
    input: FetchTransfersInput,
    startBlockId?: bigint,
  ): Promise<AccountIncomingTransfer[]> {
    if (!this.indexActor) {
      throw new Error('Cannot fetch balance without index canister id.');
    }

    const result = await this.indexActor.get_account_transactions({
      account: this.account,
      max_results: ICRC1Api.PAGE_SIZE,
      start: startBlockId ? [startBlockId] : [],
    });

    if ('Err' in result) {
      throw result.Err;
    }

    const response = result.Ok;
    let transfers: AccountIncomingTransfer[] = [];
    let nextTxId: null | bigint = null;
    if (response.transactions.length) {
      const lastTx = response.transactions[response.transactions.length - 1];
      nextTxId = lastTx.id;
    }
    response.transactions.forEach(tx => {
      if (tx.transaction.transfer[0]) {
        const transferInfo = tx.transaction.transfer[0];

        transfers.push({
          from: encodeIcrcAccount({
            owner: transferInfo.from.owner,
            subaccount: transferInfo.from.subaccount[0],
          }),
          to: encodeIcrcAccount({
            owner: transferInfo.to.owner,
            subaccount: transferInfo.to.subaccount[0],
          }),
          amount: transferInfo.amount,
          fee: transferInfo.fee[0] ?? 0n,
          created_at: nanoToJsDate(tx.transaction.timestamp),
        });
      }
    });

    if (
      transfers.length &&
      transfers[transfers.length - 1]?.created_at &&
      nextTxId !== null &&
      nextTxId !== response.oldest_tx_id?.[0]
    ) {
      const lastTransfer = transfers[transfers.length - 1];
      const lastTransferTime = lastTransfer.created_at!.getTime();
      const shouldFetchMore =
        (input.fromDt && lastTransferTime > input.fromDt!.getTime()) || (!input.fromDt && nextTxId);

      if (shouldFetchMore) {
        const moreTransfers = await this.fetchTransfers(input, nextTxId);
        transfers.push(...moreTransfers);
      }
    }

    transfers = transfers.filter(t => {
      const isInFromDt = !input.fromDt ? true : t.created_at && t.created_at >= input.fromDt;
      const isInToDt = !input.toDt ? true : t.created_at && t.created_at <= input.toDt;

      return isInFromDt && isInToDt;
    });

    return transfers;
  }

  getCapabilities(): ChainApiCapability[] {
    return [
      ChainApiCapability.Balance, // balance always available due to ledger canister id mandatory
      ...(this.indexActor ? [ChainApiCapability.Transfers] : []),
    ];
  }
}
