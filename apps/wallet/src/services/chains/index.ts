import { Asset } from '~/generated/station/station.did';
import { BlockchainStandard, BlockchainType, ChainApi } from '~/types/chain.types';
import { ICNativeApi } from './ic-native-api.service';
import { ICRC1Api } from './icrc1-api.service';
import { assertAndReturn } from '~/utils/helper.utils';

function getAssetMetadata(asset: Asset, key: string): string | undefined {
  return asset.metadata.find(m => m.key === key)?.value;
}

export class ChainApiFactory {
  static create(asset: Asset, standard: string, address: string): ChainApi {
    const chainAndStandard = `${asset.blockchain}-${standard}`;

    switch (chainAndStandard) {
      case `${BlockchainType.InternetComputer}-${BlockchainStandard.Native}`: {
        const indexCanisterId = assertAndReturn(
          getAssetMetadata(asset, 'index_canister_id'),
          'Index canister id',
        );
        return new ICNativeApi(address, indexCanisterId);
      }
      case `${BlockchainType.InternetComputer}-${BlockchainStandard.ICRC1}`: {
        const indexCanisterId = assertAndReturn(
          getAssetMetadata(asset, 'index_canister_id'),
          'Index canister id',
        );
        return new ICRC1Api(address, indexCanisterId);
      }
      default:
        throw new Error(`Blockchain not supported ${chainAndStandard}`);
    }
  }
}
