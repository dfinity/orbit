import { Actor, ActorSubclass, HttpAgent } from '@dfinity/agent';
import { appInitConfig } from '~/configs';
import { nanoToJsDate } from '~/core';
import { Account } from '~/generated/wallet/wallet.did';
import { idlFactory } from '~/generated/icp_index';
import { _SERVICE } from '~/generated/icp_index/icp_index.did';
import { FetchTransfersInput, ChainApi, AccountIncomingTransfer } from '~/types/Chain';

export class ICNativeApi implements ChainApi {
  private actor: ActorSubclass<_SERVICE>;
  static PAGE_SIZE = 10;

  constructor(private readonly account: Account) {
    this.actor = Actor.createActor<_SERVICE>(idlFactory, {
      agent: new HttpAgent({ host: appInitConfig.apiGatewayUrl.toString() }),
      canisterId: appInitConfig.canisters.icpIndex,
    });
  }

  async fetchBalance(): Promise<bigint> {
    const balance = await this.actor.get_account_identifier_balance(this.account.address);

    return balance;
  }

  async fetchTransfers(input: FetchTransfersInput): Promise<AccountIncomingTransfer[]> {
    const result = await this.actor.get_account_identifier_transactions({
      account_identifier: this.account.address,
      start: input.from_dt ? [BigInt(input.from_dt.getTime())] : [],
      max_results: BigInt(input.limit ?? ICNativeApi.PAGE_SIZE),
    });

    if ('Err' in result) {
      throw result.Err;
    }

    const response = result.Ok;
    const transfers: AccountIncomingTransfer[] = [];
    response.transactions.forEach(tx => {
      if ('Transfer' in tx.transaction.operation) {
        const transferInfo = tx.transaction.operation.Transfer;
        if (transferInfo.to !== this.account.address) {
          return;
        }

        transfers.push({
          from: transferInfo.from,
          amount: transferInfo.amount.e8s,
          fee: transferInfo.fee.e8s,
          created_at: tx.transaction.created_at_time?.[0]
            ? nanoToJsDate(tx.transaction.created_at_time[0].timestamp_nanos)
            : undefined,
        });
      }
    });

    return transfers;
  }
}
