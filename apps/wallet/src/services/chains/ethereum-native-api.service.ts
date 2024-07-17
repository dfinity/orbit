import { AccountIncomingTransfer, ChainApi, FetchTransfersInput } from '~/types/chain.types';
import { isValidSha256 } from '~/utils/helper.utils';
import { Account } from '../../generated/station/station.did';

export class EthereumNativeApi implements ChainApi {
  constructor(private readonly account: Account) {}

  isValidAddress(address: string): boolean {
    return isValidSha256(address);
  }

  async fetchBalance(): Promise<bigint> {
    return BigInt(123n);
  }

  async fetchTransfers(input: FetchTransfersInput): Promise<AccountIncomingTransfer[]> {
    return [];
  }
}
