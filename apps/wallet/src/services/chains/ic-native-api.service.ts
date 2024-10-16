import { Actor, ActorSubclass, HttpAgent } from '@dfinity/agent';
import { icAgent } from '~/core/ic-agent.core';
import { idlFactory } from '~/generated/icp_index';
import { _SERVICE } from '~/generated/icp_index/icp_index.did';
import { AccountIncomingTransfer, ChainApi, FetchTransfersInput } from '~/types/chain.types';
import { nanoToJsDate } from '~/utils/date.utils';
import { isValidSha256 } from '~/utils/helper.utils';

export class ICNativeApi implements ChainApi {
  private actor: ActorSubclass<_SERVICE>;
  static PAGE_SIZE = BigInt(100);

  constructor(
    private readonly address: string,
    private readonly indexCanisterId: string,
    agent: HttpAgent = icAgent.get(),
  ) {
    this.actor = Actor.createActor<_SERVICE>(idlFactory, {
      agent,
      canisterId: this.indexCanisterId,
    });
  }

  static isValidAddress(address: string): boolean {
    return isValidSha256(address);
  }

  isValidAddress(address: string): boolean {
    return ICNativeApi.isValidAddress(address);
  }

  async fetchBalance(): Promise<bigint> {
    const balance = await this.actor.get_account_identifier_balance(this.address);

    return balance;
  }

  async fetchTransfers(
    input: FetchTransfersInput,
    startBlockId?: bigint,
  ): Promise<AccountIncomingTransfer[]> {
    const result = await this.actor.get_account_identifier_transactions({
      account_identifier: this.address,
      start: startBlockId ? [startBlockId] : [],
      max_results: ICNativeApi.PAGE_SIZE,
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
      if ('Transfer' in tx.transaction.operation) {
        const transferInfo = tx.transaction.operation.Transfer;

        transfers.push({
          from: transferInfo.from,
          to: transferInfo.to,
          amount: transferInfo.amount.e8s,
          fee: transferInfo.fee.e8s,
          created_at: tx.transaction.created_at_time?.[0]
            ? nanoToJsDate(tx.transaction.created_at_time[0].timestamp_nanos)
            : undefined,
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
