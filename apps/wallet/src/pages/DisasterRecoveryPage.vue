<template>
  <PageLayout>
    <template #main-body>
      <VCard class="ma-4">
        <div class="d-flex flex-row flex-no-wrap justify-space-between">
          <div class="flex-grow-1 my-4">
            <VCardTitle class="text-h4 text-wrap">
              {{ $t('pages.disaster_recovery.title') }}
            </VCardTitle>
            <VCardSubtitle class="text-wrap">
              {{ $t('pages.disaster_recovery.subtitle') }}
            </VCardSubtitle>

            <template v-if="state.name === 'loading_upgrader'">
              <div class="d-flex flex-column flex-no-wrap align-center">
                <VProgressCircular
                  class="mt-10"
                  color="primary"
                  indeterminate
                  size="90"
                  width="8"
                />
                <VCardText class="text-wrap">
                  {{ $t('pages.disaster_recovery.loading_upgrader') }}
                </VCardText>
              </div>
            </template>

            <template v-else-if="state.name === 'loading_state'">
              <div class="d-flex flex-column flex-no-wrap align-center">
                <VProgressCircular
                  class="mt-10"
                  color="primary"
                  indeterminate
                  size="90"
                  width="8"
                />
                <VCardText class="text-wrap">
                  {{ $t('pages.disaster_recovery.loading_state') }}
                </VCardText>
              </div>
            </template>

            <template v-else-if="state.name === 'submitting_recovery'">
              <VCardText class="text-wrap">
                Upgrader: {{ state.upgrader.upgrader.toText() }}
              </VCardText>
              <VCardText class="text-wrap">
                {{ $t('pages.disaster_recovery.submitting_recovery') }}
              </VCardText>

              <VCardText>
                <VSelect
                  v-if="state.registryState.name === 'loaded_registry'"
                  :items="state.registryState.registry"
                  item-title="value.WasmModule.version"
                  item-value="id"
                  placeholder="Select Orbit station version"
                  v-model="selectedRegistry"
                  :return-object="true"
                  hide-details
                />
              </VCardText>

              <div
                class="d-flex flex-row flex-no-wrap justify-space-between"
                v-if="state.wasm?.wasmIdl"
              >
                <VCardText>
                  <VLabel>Station IDL</VLabel>
                  <VTextarea
                    v-model="state.wasm.wasmIdl"
                    density="compact"
                    :rows="16"
                    class="font-monospace small-font no-wrap"
                    ref="stationIdlTextarea"
                    readonly
                  ></VTextarea>
                </VCardText>
                <VCardText>
                  <VLabel>Disaster Recovery State</VLabel>
                  <VTextarea
                    :rows="16"
                    class="font-monospace small-font no-wrap"
                    density="compact"
                    v-model="humanReadableState"
                    readonly
                  ></VTextarea>
                </VCardText>
              </div>

              <VCardText>
                <CanisterArgumentField
                  v-if="state.wasm"
                  ref="canisterArgumentField"
                  v-model="payload"
                  required
                  name="argument"
                  :candid="{
                    idl: state.wasm?.wasmIdl ?? '',
                    withType: { serviceParams: null },
                  }"
                  :rows="10"
                />
              </VCardText>

              <div
                class="d-flex flex-row flex-no-wrap justify-space-between"
                v-if="state.wasm?.wasmIdl"
              >
                <VCardText v-if="state.wasm">
                  <VLabel>Upgrader IDL</VLabel>
                  <VTextarea
                    v-model="state.upgrader.candid"
                    density="compact"
                    :rows="16"
                    class="font-monospace small-font no-wrap"
                    ref="upgraderIdlTextarea"
                    readonly
                  ></VTextarea>
                </VCardText>
                <VCardText>
                  <VLabel>Install Mode Payload</VLabel>
                  <VTextarea
                    v-model="payloadHumanReadable"
                    density="compact"
                    :rows="16"
                    class="font-monospace small-font"
                    readonly
                  ></VTextarea>
                </VCardText>
              </div>

              <VCardText>
                <CanisterArgumentField
                  v-if="state.wasm"
                  ref="drRequestPayloadField"
                  v-model="drRequestPayload"
                  required
                  name="argument"
                  :candid="{
                    idl: state.upgrader.candid,
                    withType: { methodParams: 'request_disaster_recovery' },
                  }"
                  :rows="10"
                />
              </VCardText>
            </template>

            <VCardActions>
              <VBtn
                v-if="state.name === 'submitting_recovery'"
                color="primary"
                size="large"
                block
                :loading="state.submitLoading"
                :disabled="!drRequestPayload"
                @click="submitRecovery"
              >
                {{ $t('pages.disaster_recovery.submit_button') }}
              </VBtn>
            </VCardActions>
          </div>
        </div>
      </VCard>
    </template>
  </PageLayout>
</template>

<style>
.small-font textarea {
  font-size: 0.8rem !important;
}

.no-wrap textarea {
  white-space: pre;
  overflow-wrap: normal;
  overflow-x: scroll;
}
</style>

<script setup lang="ts">
import { computed, nextTick, onBeforeUnmount, onMounted, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import {
  VBtn,
  VCard,
  VCardActions,
  VCardSubtitle,
  VCardText,
  VCardTitle,
  VProgressCircular,
  VTextarea,
} from 'vuetify/components';
import CanisterArgumentField from '~/components/inputs/CanisterArgumentField.vue';
import PageLayout from '~/components/PageLayout.vue';
import { fetchCanisterControllers } from '~/utils/helper.utils';
import { icAgent } from '~/core/ic-agent.core';
import { Principal } from '@dfinity/principal';
import { useStationStore } from '~/stores/station.store';
import { fetchCanisterIdlFromMetadata } from '~/utils/didc.utils';
import { UpgraderService } from '~/services/upgrader.service';
import {
  // Account,
  AdminUser,
  Asset,
  GetDisasterRecoveryStateResponse,
  MultiAssetAccount,
} from '~/generated/upgrader/upgrader.did';
import { RegistryEntry, WasmModuleExtraChunks } from '~/generated/control-panel/control_panel.did';
import { services } from '~/plugins/services.plugin';
import { appInitConfig } from '~/configs/init.config';
import { gunzip } from 'fflate';
import { focusText } from '~/utils/form.utils';

const recoveryPayload = ref('');

const i18n = useI18n();

// const installModes = [
//   {
//     label: 'Install in empty station canister',
//     value: 'install',
//   },
//   {
//     label: 'Upgrade existing station canister',
//     value: 'upgrade',
//   },
//   {
//     label: 'Reinstall clean station canister',
//     value: 'reinstall',
//   },
// ];

type UpgraderInfo = {
  upgrader: Principal;
  candid: string;
};

const payload = ref<Uint8Array>();
const station = useStationStore();

const payloadHumanReadable = computed(() => {
  if (payload.value) {
    return `blob "${blobToHumanReadable(payload.value)}"`;
  }
  return null;
});

const drRequestPayload = ref<Uint8Array>();

const controlPanelService = services().controlPanel;
const unmounted = ref(false);

const selectedRegistry = ref<RegistryEntry | null>(null);

let upgraderService: UpgraderService | null = null;

type RegistryState =
  | {
      name: 'loading_registry';
      isLoading: boolean;
      error: string;
    }
  | {
      name: 'loaded_registry';
      registry: RegistryEntry[];
    };

type WasmState = {
  wasm: Uint8Array | null;
  wasmIdl: string | null;
  extraChunks: WasmModuleExtraChunks;
};

type ConnectionState =
  | {
      name: 'loading_upgrader';
      isLoading: boolean;
      error: string;
    }
  | {
      name: 'loading_state';
      upgrader: UpgraderInfo;
      // service: UpgraderService;
      stateLoading: boolean;
      error: string;
    }
  | {
      name: 'submitting_recovery';
      upgrader: UpgraderInfo;
      // service: UpgraderService;
      disasterRecoveryState: GetDisasterRecoveryStateResponse;
      registryState: RegistryState;
      submitLoading: boolean;
      error: string;
      payload: Uint8Array;
      wasm: WasmState | null;
    };

const state = ref<ConnectionState>({
  name: 'loading_upgrader',
  isLoading: true,
  error: '',
});

const stationIdlTextarea = ref<InstanceType<typeof VTextarea>>();

function blobToHumanReadable(blob: Uint8Array | number[]): string {
  // convert to hex string with 2 digits per byte, each escaped with \xx
  return Array.from(blob)
    .map(b => '\\' + b.toString(16).padStart(2, '0'))
    .join('');
}

function stateToHumanReadable(state: GetDisasterRecoveryStateResponse): string {
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

const humanReadableState = computed(() => {
  if (state.value.name === 'submitting_recovery') {
    return stateToHumanReadable(
      state.value.disasterRecoveryState as GetDisasterRecoveryStateResponse,
    );
  }

  return '';
});

function committeeUserToInitUser(user: AdminUser): string {
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

function accountToInitAccount(account: MultiAssetAccount) {
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

function assetToInitAsset(asset: Asset) {
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

function indent(str: string, indentLevel: number, indentSize = 2): string {
  return str
    .split('\n')
    .map(line => ' '.repeat(indentLevel * indentSize) + line)
    .join('\n');
}

function systemInstallArgs(upgraderId: Principal, drState: GetDisasterRecoveryStateResponse) {
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

function drRequestArgs(payloadHumanReadable: string, extraChunks: WasmModuleExtraChunks) {
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

const canisterArgumentField = ref<InstanceType<typeof CanisterArgumentField>>();
const drRequestPayloadField = ref<InstanceType<typeof CanisterArgumentField>>();
watch(
  [
    () =>
      (state.value.name === 'submitting_recovery' ? state.value.upgrader.upgrader : undefined) as
        | Principal
        | undefined,
    () => (state.value.name === 'submitting_recovery' ? state.value.wasm?.wasmIdl : undefined),
    () =>
      state.value.name === 'submitting_recovery' ? state.value.disasterRecoveryState : undefined,
    canisterArgumentField,
  ],
  ([upgraderId, wasmIdl, disasterRecoveryState, canisterArgumentField]) => {
    if (upgraderId && wasmIdl && disasterRecoveryState && canisterArgumentField) {
      canisterArgumentField.setArgument(
        systemInstallArgs(upgraderId, disasterRecoveryState as GetDisasterRecoveryStateResponse),
      );
    }
  },
);

watch(
  [
    payloadHumanReadable,
    () => (state.value.name === 'submitting_recovery' ? state.value.wasm : undefined),
  ],
  ([payloadHumanReadable, wasm]) => {
    if (payloadHumanReadable && wasm) {
      drRequestPayloadField.value?.setArgument(
        drRequestArgs(payloadHumanReadable, wasm.extraChunks as WasmModuleExtraChunks),
      );
    }
  },
);

watch(selectedRegistry, async newSelectedRegistry => {
  // console.log(newSelectedRegistry);

  if (newSelectedRegistry) {
    const wasm = await downloadWasm(newSelectedRegistry as RegistryEntry);

    if (state.value.name === 'submitting_recovery') {
      state.value = {
        ...state.value,
        wasm: wasm,
      };
      await nextTick();
      await nextTick();
      await nextTick();
      await nextTick();
      focusText(stationIdlTextarea.value?.$el.querySelector('textarea'), 'type SystemInstall');
    }
  }
});

async function getUpgrader(): Promise<UpgraderInfo> {
  const controllers = await fetchCanisterControllers(
    icAgent.get(),
    Principal.fromText(station.canisterId),
  );

  for (const controller of controllers ?? []) {
    const candid = await fetchCanisterIdlFromMetadata(controller, icAgent.get());
    if (candid?.includes('request_disaster_recovery')) {
      return {
        upgrader: controller,
        candid,
      };
    }
  }

  throw new Error(i18n.t('pages.disaster_recovery.error_no_upgrader_found'));
}

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

async function downloadWasm(registryEntry: RegistryEntry): Promise<WasmState> {
  // const { artifact } = await controlPanelService.getArtifact({
  //   artifact_id: registryEntry.value.WasmModule.wasm_artifact_id,
  // });
  // console.log(artifact);

  // console.log(registryEntry);

  const extraChunks = registryEntry.value.WasmModule.module_extra_chunks[0];

  console.log(extraChunks);

  if (extraChunks) {
    const url = appInitConfig.httpGatewayUrl(extraChunks?.store_canister.toText());
    url.pathname = extraChunks.extra_chunks_key;

    const response = await fetch(url);
    const blob = await response.blob();

    const wasm = await unzipBlob(blob);

    const mod = await WebAssembly.compile(wasm);

    const candid = WebAssembly.Module.customSections(mod, 'icp:public candid:service');

    // turn to string
    const candidString = new TextDecoder().decode(candid[0]);

    // decode({idl: candidString, })

    // console.log(candidString);

    // return candidString;

    return {
      wasm: wasm,
      wasmIdl: candidString,
      extraChunks,
    };
  } else {
    throw new Error('No extra chunks found');
  }

  // const bin =

  // await fetch( extraChunks.)

  // return new Uint8Array(artifact.artifact.artifact);
}

async function submitRecovery() {
  if (state.value.name === 'submitting_recovery' && drRequestPayload.value) {
    // state.value.submitLoading = true;
    await upgraderService!.submitRecovery(drRequestPayload.value);
  }
}

onMounted(async () => {
  state.value = {
    name: 'loading_upgrader',
    isLoading: true,
    error: '',
  };

  try {
    const upgraderInfo = await getUpgrader();
    upgraderService = new UpgraderService(icAgent.get(), upgraderInfo.upgrader);
    state.value = {
      name: 'loading_state',
      upgrader: upgraderInfo,
      // service: new UpgraderService(icAgent.get(), upgraderInfo.upgrader),
      stateLoading: true,
      error: '',
    };
  } catch (error) {
    state.value = {
      name: 'loading_upgrader',
      isLoading: false,
      error: i18n.t('pages.disaster_recovery.error_state', { error: error }),
    };
    return;
  } finally {
    if (unmounted.value) return;
  }

  try {
    const drState = await upgraderService!.getDisasterRecoveryState();

    state.value = {
      name: 'submitting_recovery',
      upgrader: state.value.upgrader,
      // service: state.value.service,
      disasterRecoveryState: drState,
      registryState: {
        name: 'loading_registry',
        isLoading: true,
        error: '',
      },
      submitLoading: false,
      error: '',
      payload: new Uint8Array(),
      wasm: null,
    };
  } catch (error) {
    state.value = {
      name: 'loading_state',
      upgrader: state.value.upgrader,
      // service: state.value.service,
      stateLoading: false,
      error: i18n.t('pages.disaster_recovery.error_state_loading_failed', { error: error }),
    };
    return;
  } finally {
    if (unmounted.value) return;
  }

  try {
    const registryEntries = await controlPanelService.findRegistryEntries({
      filter_by: [{ Name: '@orbit/station' }],
      pagination: [],
      sort_by: [],
    });

    state.value = {
      ...state.value,
      registryState: {
        name: 'loaded_registry',
        registry: registryEntries.entries,
      },
      payload: new Uint8Array(),
    };
  } catch (error) {
    state.value = {
      ...state.value,
      registryState: {
        name: 'loading_registry',
        error: i18n.t('pages.disaster_recovery.error_registry_loading_failed', { error: error }),
        isLoading: false,
      },
    };

    return;
  } finally {
    if (unmounted.value) return;
  }
});

onBeforeUnmount(() => {
  unmounted.value = true;
});
</script>
