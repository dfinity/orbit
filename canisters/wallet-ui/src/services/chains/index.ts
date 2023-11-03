import { Account } from '~/generated/bank/bank.did';
import { ICNativeApi } from './ICNativeApi';
import { BlockchainStandard, BlockchainType, ChainApi } from '~/types/Chain';

export class ChainApiFactory {
  static create(account: Account): ChainApi {
    const chainAndStandard = `${account.blockchain}-${account.standard}`;

    switch (chainAndStandard) {
      case `${BlockchainType.InternetComputer}-${BlockchainStandard.Native}`:
        return new ICNativeApi(account);
      default:
        throw new Error(`Blockchain not supported ${chainAndStandard}`);
    }
  }
}
