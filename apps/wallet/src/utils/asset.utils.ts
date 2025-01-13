import { decodeIcrcAccount } from '@dfinity/ledger-icrc';
import { ckTokens } from '~/configs/ck-tokens.config';
import { appInitConfig } from '~/configs/init.config';
import { Asset, StandardData, SupportedBlockchain } from '~/generated/station/station.did';
import { services } from '~/plugins/services.plugin';
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
    const subaccount = address.split('.')[1];

    if (subaccount.length <= 27) {
      // the subaccount is short enough to show the whole thing
      return `${address.slice(0, 6)}...${subaccount}`;
    }

    // shorted the subaccount
    return `${address.slice(0, 6)}...${subaccount.slice(0, 20)}...${address.slice(-4)}`;
  }
}

export type WellKnownAsset = Omit<Asset, 'id'>;

type NatMetadata = { Nat: [number] };

type TextMetadata = { Text: string };

// taken from https://github.com/dfinity/ic-js/blob/a98fa59bcd653887a2e89c85e6d7700bad2abdff/packages/nns-proto/proto/swap.proto#L17
enum SNSAggregatorAssetLifecycle {
  UNSPECIFIED = 0,
  PENDING = 1,
  ADOPTED = 5,
  OPEN = 2,
  COMMITTED = 3,
  ABORTED = 4,
}

type SNSAggregatorAsset = {
  canister_ids: {
    ledger_canister_id: string;
    index_canister_id: string;
  };
  meta: {
    url: string;
    name: string;
    description: string;
    logo: string;
  };
  lifecycle: {
    lifecycle: SNSAggregatorAssetLifecycle;
  };
  icrc1_metadata: (
    | ['icrc1:decimals', NatMetadata]
    | ['icrc1:name', TextMetadata]
    | ['icrc1:symbol', TextMetadata]
    | ['icrc1:fee', NatMetadata]
  )[];
};

function getAggregatorAssetMetadata<T>(
  metadata: SNSAggregatorAsset['icrc1_metadata'],
  key: string,
): T | undefined {
  return metadata.find(([k]) => k === key)?.[1] as T | undefined;
}

function fetchAggregatorPage(page: number): Promise<SNSAggregatorAsset[]> {
  if (appInitConfig.isProduction) {
    return fetch(
      `https://3r4gx-wqaaa-aaaaq-aaaia-cai.icp0.io/v1/sns/list/page/${page}/slow.json`,
    ).then(res => res.json() as Promise<SNSAggregatorAsset[]>);
  }
  return import('../mocks/aggregator-slow.json').then(m => m.default) as Promise<
    SNSAggregatorAsset[]
  >;
}

export async function fetchSNSAssets(): Promise<SNSAggregatorAsset[]> {
  const MAX_RETRIES = 5;
  const allDaos: SNSAggregatorAsset[] = [];
  let page = 0;
  let retriesLeft = MAX_RETRIES;
  while (true) {
    try {
      const pageData = await fetchAggregatorPage(page);
      allDaos.push(...pageData);
      if (pageData.length < 10) {
        break;
      }
      page++;
      retriesLeft = MAX_RETRIES;
    } catch (e) {
      if (retriesLeft > 0) {
        await new Promise(r => setTimeout(r, 60 * 1000));
        retriesLeft--;
        continue;
      } else {
        throw e;
      }
    }
  }

  allDaos.sort((a, b) => {
    if (a.meta.name < b.meta.name) {
      return -1;
    }
    if (a.meta.name > b.meta.name) {
      return 1;
    }
    return 0;
  });

  return allDaos;
}

export async function fetchCkTokens(): Promise<WellKnownAsset[]> {
  const tokens = [...ckTokens];

  tokens.sort((a, b) => {
    if (a.name < b.name) {
      return -1;
    }
    if (a.name > b.name) {
      return 1;
    }
    return 0;
  });

  return tokens;
}

export type GroupedWellKnownAssets = { groupKey: string; assets: WellKnownAsset[] }[];

export async function fetchWellKnownIcpAssets(): Promise<GroupedWellKnownAssets> {
  const snsAssets = await fetchSNSAssets();
  const ckTokens = await fetchCkTokens();
  const maybeSnsAssets = snsAssets.map(asset => {
    if (asset.lifecycle.lifecycle !== 3) {
      // only 3=COMMITTED SNS assets exist
      return;
    }

    const decimals = getAggregatorAssetMetadata<NatMetadata>(asset.icrc1_metadata, 'icrc1:decimals')
      ?.Nat[0];
    const symbol = getAggregatorAssetMetadata<TextMetadata>(
      asset.icrc1_metadata,
      'icrc1:symbol',
    )?.Text;
    const name = getAggregatorAssetMetadata<TextMetadata>(asset.icrc1_metadata, 'icrc1:name')?.Text;
    const fee = getAggregatorAssetMetadata<NatMetadata>(asset.icrc1_metadata, 'icrc1:fee')?.Nat[0];

    if (decimals === undefined || symbol === undefined || name === undefined || fee === undefined) {
      return;
    }

    return {
      blockchain: 'icp',
      standards: ['icrc1'],
      symbol,
      decimals,
      metadata: [
        { key: 'ledger_canister_id', value: asset.canister_ids.ledger_canister_id },
        { key: 'index_canister_id', value: asset.canister_ids.index_canister_id },
        { key: 'url', value: asset.meta.url },
        { key: 'description', value: asset.meta.description },
        { key: 'logo', value: asset.meta.logo },
      ],
      name,
    };
  });

  return [
    {
      groupKey: 'ck_tokens',
      assets: ckTokens,
    },
    {
      groupKey: 'sns',
      assets: maybeSnsAssets.filter((a): a is WellKnownAsset => !!a),
    },
  ];
}

export async function getAllAssets() {
  const allAssets: Asset[] = [];
  let nextOffset: number | undefined;
  do {
    const result = await services().station.listAssets({
      limit: 100,
      offset: nextOffset,
    });
    allAssets.push(...result.assets);

    nextOffset = result.next_offset[0] ? Number(result.next_offset[0]) : undefined;
  } while (nextOffset !== undefined);

  return allAssets;
}
