import { Account } from '~/generated/wallet/wallet.did';
import { BlockchainStandard, BlockchainType, ChainApi } from '~/types/chain.types';
import { ICNativeApi } from './ic-native-api.service';

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
