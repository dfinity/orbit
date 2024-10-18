import { AccountAddress, Asset } from '~/generated/station/station.did';
import { AddressFormat, BlockchainStandard, BlockchainType, ChainApi } from '~/types/chain.types';
import { getAssetMetadata } from '~/utils/asset.utils';
import { ICNativeApi } from './ic-native-api.service';
import { ICRC1Api } from './icrc1-api.service';

export class ChainApiFactory {
  static create(asset: Asset, addresses: AccountAddress[]): ChainApi {
    switch (asset.blockchain) {
      case BlockchainType.InternetComputer: {
        const maybeIcpNativeAddress = addresses.find(a => a.format === AddressFormat.ICPNative);
        const maybeIcrc1Address = addresses.find(a => a.format === AddressFormat.ICRC1);
        const maybeIndexCanisterId = getAssetMetadata(asset, 'index_canister_id');

        if (
          asset.standards.includes(BlockchainStandard.Native) &&
          maybeIcpNativeAddress &&
          maybeIndexCanisterId
        ) {
          return new ICNativeApi(maybeIcpNativeAddress.address, maybeIndexCanisterId);
        }

        if (
          asset.standards.includes(BlockchainStandard.ICRC1) &&
          maybeIcrc1Address &&
          maybeIndexCanisterId
        ) {
          return new ICRC1Api(maybeIcrc1Address.address, maybeIndexCanisterId);
        }

        throw new Error(`Blockchain not supported: ${asset.blockchain}`);
      }
      case BlockchainType.Bitcoin:
      case BlockchainType.Ethereum:
      default:
        throw new Error(`Blockchain not supported: ${asset.blockchain}`);
    }

    // const chainAndStandard = `${asset.blockchain}-${standard}`;

    // switch (chainAndStandard) {
    //   case `${BlockchainType.InternetComputer}-${BlockchainStandard.Native}`: {
    //     const indexCanisterId = assertAndReturn(
    //       getAssetMetadata(asset, 'index_canister_id'),
    //       'Index canister id',
    //     );
    //     return new ICNativeApi(address, indexCanisterId);
    //   }
    //   case `${BlockchainType.InternetComputer}-${BlockchainStandard.ICRC1}`: {
    //     const indexCanisterId = assertAndReturn(
    //       getAssetMetadata(asset, 'index_canister_id'),
    //       'Index canister id',
    //     );
    //     return new ICRC1Api(address, indexCanisterId);
    //   }
    //   default:
    //     throw new Error(`Blockchain not supported ${chainAndStandard}`);
    // }
  }
}
