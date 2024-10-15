import { Actor, ActorSubclass, HttpAgent } from '@dfinity/agent';
import { icAgent } from '~/core/ic-agent.core';
import { idlFactory } from '~/generated/icrc1_index';
import { _SERVICE } from '~/generated/icrc1_index/icrc1_index_canister.did';
import { AccountIncomingTransfer, ChainApi, FetchTransfersInput } from '~/types/chain.types';
import { nanoToJsDate } from '~/utils/date.utils';
import { decodeIcrcAccount, encodeIcrcAccount } from '@dfinity/ledger-icrc';
import { Account } from '~/generated/icp_index/icp_index.did';

export class ICRC1Api implements ChainApi {
  private actor: ActorSubclass<_SERVICE>;
  static PAGE_SIZE = BigInt(100);

  private account: Account;

  constructor(
    address: string,
    private readonly indexCanisterId: string,
    agent: HttpAgent = icAgent.get(),
  ) {
    const icrc1Account = decodeIcrcAccount(address);

    this.account = {
      owner: icrc1Account.owner,
      subaccount: icrc1Account.subaccount ? [icrc1Account.subaccount] : [],
    };

    this.actor = Actor.createActor<_SERVICE>(idlFactory, {
      agent,
      canisterId: this.indexCanisterId,
    });
  }

  isValidAddress(address: string): boolean {
    try {
      decodeIcrcAccount(address);
      return true;
    } catch {
      return false;
    }
  }

  async fetchBalance(): Promise<bigint> {
    const balance = await this.actor.icrc1_balance_of(this.account);

    return balance;
  }

  async fetchTransfers(
    input: FetchTransfersInput,
    startBlockId?: bigint,
  ): Promise<AccountIncomingTransfer[]> {
    const result = await this.actor.get_account_transactions({
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
}
