import { decodeIcrcAccount } from '@dfinity/ledger-icrc';
import { Asset, StandardData, SupportedBlockchain } from '~/generated/station/station.did';
import { ICNativeApi } from '~/services/chains/ic-native-api.service';
import { ICRC1Api } from '~/services/chains/icrc1-api.service';
import { AddressFormat, BlockchainType } from '~/types/chain.types';

export function getAssetMetadata(asset: Asset, key: string): string | undefined {
  return asset.metadata.find(m => m.key === key)?.value;
}

export function detectAddressFormat(blockchain: string, address: string): string | undefined {
  switch (blockchain) {
    case BlockchainType.InternetComputer:
      if (ICNativeApi.isValidAddress(address)) {
        return AddressFormat.ICPNative;
      } else if (ICRC1Api.isValidAddress(address)) {
        return AddressFormat.ICRC1;
      } else {
        return;
      }
    case BlockchainType.Bitcoin:
    case BlockchainType.Ethereum:
      return;
    default:
      throw new Error(`Blockchain not supported ${blockchain}`);
  }
}

export function detectAddressStandard(
  asset: Asset,
  address: string,
  supportedBlockchains: SupportedBlockchain[],
): StandardData | undefined {
  const maybeFormat = detectAddressFormat(asset.blockchain, address);
  if (!maybeFormat) {
    return;
  }

  const supportedStandards = supportedBlockchains
    .find(b => b.blockchain === asset.blockchain)
    ?.supported_standards.filter(supportedStandard =>
      asset.standards.includes(supportedStandard.standard),
    );

  return supportedStandards?.find(s => s.supported_address_formats.includes(maybeFormat));
}

export function shortenIcrc1Address(address: string): string {
  const account = decodeIcrcAccount(address);
  const principal = account.owner.toText();

  if (!account.subaccount || account.subaccount.every(b => b === 0)) {
    // show just the principal, if there is no subaccount
    if (principal.length <= 32) {
      // the principal is short enough to show the whole thing
      return principal;
    }

    // shorten the principal
    return principal.slice(0, 12) + '...' + principal.slice(-10);
  } else {
    let subaccount = address.split('.')[1];

    if (subaccount.length <= 27) {
      // the subaccount is short enough to show the whole thing
      return `${address.slice(0, 6)}...${subaccount}`;
    }

    // shorted the subaccount
    return `${address.slice(0, 6)}...${subaccount.slice(0, 20)}...${address.slice(-4)}`;
  }
}
