import { Principal } from '@dfinity/principal';
import { gunzip } from 'fflate';
import { appInitConfig } from '~/configs/init.config';
import { RegistryEntry, WasmModuleExtraChunks } from '~/generated/control-panel/control_panel.did';
import {
  AdminUser,
  Asset,
  GetDisasterRecoveryStateResponse,
  MultiAssetAccount,
} from '~/generated/upgrader/upgrader.did';

async function unzipBlob(blob: Blob): Promise<Uint8Array> {
  return new Promise((resolve, reject) => {
    blob.arrayBuffer().then(arrayBuffer => {
      gunzip(new Uint8Array(arrayBuffer), (err, data) => {
        if (err) reject(err);
        resolve(data);
      });
    });
  });
}

export type DownloadedWasm = {
  wasm: Uint8Array | null;
  wasmIdl: string | null;
  extraChunks: WasmModuleExtraChunks;
};

export async function downloadRegistryEntry(registryEntry: RegistryEntry): Promise<DownloadedWasm> {
  const extraChunks = registryEntry.value.WasmModule.module_extra_chunks[0];

  if (extraChunks) {
    const url = appInitConfig.httpGatewayUrl(extraChunks?.store_canister.toText());
    url.pathname = extraChunks.extra_chunks_key;

    const response = await fetch(url);
    const blob = await response.blob();
    const wasm = await unzipBlob(blob);
    const mod = await WebAssembly.compile(wasm);
    const candid = WebAssembly.Module.customSections(mod, 'icp:public candid:service');
    const candidString = new TextDecoder().decode(candid[0]);

    return {
      wasm: wasm,
      wasmIdl: candidString,
      extraChunks,
    };
  } else {
    throw new Error('No extra chunks found');
  }
}

export function committeeUserToInitUser(user: AdminUser): string {
  return `record {
    id = opt "${user.id}";
    name = "${user.name}";
    identities = vec {
      record {
        identity = principal "${user.identities[0]?.toText()}"
      }
    };
    status = variant { Active };
  }`;
}

export function accountToInitAccount(account: MultiAssetAccount) {
  return `record {
    id = opt "${account.id}";
    name = "${account.name}";
    seed = blob "${blobToHumanReadable(account.seed)}";
    assets = vec {
      ${account.assets.map(asset => `"${asset}"`).join(';\n')}
    };
    metadata = vec {
      ${account.metadata
        .map(
          metadata => `record {
        key = "${metadata.key}";
        value = "${metadata.value}";
      }`,
        )
        .join('; ')}
    }
  }`;
}

export function assetToInitAsset(asset: Asset) {
  return `record {
    id = opt "${asset.id}";
    name = "${asset.name}";
    decimals = ${asset.decimals};
    symbol = "${asset.symbol}";
    standards = vec {
      ${asset.standards.map(standard => `"${standard}"`).join(';\n')}
    };
    blockchain = "${asset.blockchain}";
    metadata = vec {
      ${asset.metadata
        .map(
          metadata => `record {
        key = "${metadata.key}";
        value = "${metadata.value}";
      }`,
        )
        .join(';\n')}
    }
  }`;
}

export function indent(str: string, indentLevel: number, indentSize = 2): string {
  return str
    .split('\n')
    .map(line => ' '.repeat(indentLevel * indentSize) + line)
    .join('\n');
}

export function systemInstallArgs(
  upgraderId: Principal,
  drState: GetDisasterRecoveryStateResponse,
) {
  const adminQuorum = drState.committee[0]?.quorum;
  const operatorQuorum = 1;

  return `(opt variant {
    Init = record {
      name = "Orbit Station";
      upgrader = variant {
        Id = principal "${upgraderId.toText()}"
      };
      fallback_controller = opt principal "r7inp-6aaaa-aaaaa-aaabq-cai"; // NNS Root
      initial_config = variant {
        WithDefaultPolicies = record {
          users = vec {
  ${drState.committee[0]?.users.map(user => indent(committeeUserToInitUser(user), 5)).join(';\n')}
          };
          accounts = vec {
  ${drState.multi_asset_accounts.map(account => indent(accountToInitAccount(account), 5)).join(';\n')}
          };
          assets = vec {
  ${drState.assets.map(asset => indent(assetToInitAsset(asset), 5)).join(';\n')}
          };
          admin_quorum = ${adminQuorum};
          operator_quorum = ${operatorQuorum};
        }
      }
    }
  })`;
}

export function drRequestArgs(payloadHumanReadable: string, extraChunks: WasmModuleExtraChunks) {
  return `(variant {
      InstallCode = record {
        module = blob "";
        module_extra_chunks = opt record {
          store_canister = principal "${extraChunks.store_canister.toText()}";
          extra_chunks_key = "${extraChunks.extra_chunks_key}";
          wasm_module_hash = blob "${blobToHumanReadable(extraChunks.wasm_module_hash)}";
        } ;
        arg = ${payloadHumanReadable};
        install_mode = variant { Reinstall };
      }
    })`;
}

export function blobToHumanReadable(blob: Uint8Array | number[]): string {
  // convert to hex string with 2 digits per byte, each escaped with \xx
  return Array.from(blob)
    .map(b => '\\' + b.toString(16).padStart(2, '0'))
    .join('');
}

export function stateToHumanReadable(state: GetDisasterRecoveryStateResponse): string {
  let result = 'Committee (quorum=' + state.committee[0]?.quorum + '):';

  for (const user of state.committee[0]?.users ?? []) {
    result += `\n  - name: ${user.name}`;
    result += `\n    id: "${user.id}"`;
    result += `\n    identities:`;

    for (const identity of user.identities ?? []) {
      result += `\n      - principal "${identity.toText()}"`;
    }
  }

  if (state.multi_asset_accounts.length > 0 || state.accounts.length > 0) {
    result += `\n\nAccounts:`;

    for (const account of state.multi_asset_accounts ?? []) {
      result += `\n  - name: ${account.name}`;
      result += `\n    id: "${account.id}"`;

      result += `\n    seed: blob "${blobToHumanReadable(account.seed)}"`;

      if (account.assets.length > 0) {
        result += `\n    assets:`;

        for (const assetId of account.assets ?? []) {
          result += `\n      - "${assetId}"`;
        }
      }

      if (account.metadata.length > 0) {
        result += `\n    metadata:`;

        for (const metadata of account.metadata) {
          result += `\n      - "${metadata.key}": "${metadata.value}"`;
        }
      }
    }

    for (const account of state.accounts ?? []) {
      result += `\n  - name: ${account.name}`;
      result += `\n    id: "${account.id}"`;
      result += `\n    blockchain: ${account.blockchain}`;
      result += `\n    address: ${account.address}`;
      result += `\n    standard: ${account.standard}`;
      result += `\n    symbol: ${account.symbol}`;
      result += `\n    decimals: ${account.decimals}`;
      if (account.metadata.length > 0) {
        result += `\n    metadata:`;

        for (const metadata of account.metadata) {
          result += `\n      - "${metadata.key}": "${metadata.value}"`;
        }
      }
    }
  }

  if (state.assets.length > 0) {
    result += `\n\nAssets:`;

    for (const asset of state.assets ?? []) {
      result += `\n  - name: ${asset.name}`;
      result += `\n    id: "${asset.id}"`;
      result += `\n    decimals: ${asset.decimals}`;
      result += `\n    symbol: ${asset.symbol}`;
      result += `\n    standards: ${asset.standards.join(', ')}`;
      result += `\n    blockchain: ${asset.blockchain}`;
      result += `\n    metadata:`;

      for (const metadata of asset.metadata) {
        result += `\n      - "${metadata.key}": "${metadata.value}"`;
      }
    }
  }

  return result;
}
