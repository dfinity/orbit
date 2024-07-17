import { Account } from '~/generated/station/station.did';
import { BlockchainStandard, BlockchainType, ChainApi } from '~/types/chain.types';
import { EthereumNativeApi } from './ethereum-native-api.service';
import { ICNativeApi } from './ic-native-api.service';

export class ChainApiFactory {
  static create(account: Account): ChainApi {
    const chainAndStandard = `${account.blockchain}-${account.standard}`;

    switch (chainAndStandard) {
      case `${BlockchainType.InternetComputer}-${BlockchainStandard.Native}`:
        return new ICNativeApi(account);
      case `${BlockchainType.Ethereum}-${BlockchainStandard.Native}`:
        return new EthereumNativeApi(account);
      default:
        throw new Error(`Blockchain not supported ${chainAndStandard}`);
    }
  }
}
