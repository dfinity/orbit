import { Wallet } from '~/generated/bank/bank.did';
import { ICNativeApi } from './ICNativeApi';
import { BlockchainStandard, BlockchainType, WalletApi } from '~/types/Wallet';

export class WalletApiFactory {
  static create(wallet: Wallet): WalletApi {
    const chainAndStandard = `${wallet.blockchain}-${wallet.standard}`;

    switch (chainAndStandard) {
      case `${BlockchainType.InternetComputer}-${BlockchainStandard.Native}`:
        return new ICNativeApi(wallet);
      default:
        throw new Error(`Blockchain not supported ${chainAndStandard}`);
    }
  }
}
