<template>
  <PageLayout>
    <template #main-header>
      <PageHeader :title="$t('pages.disaster_recovery.title')" />
    </template>
    <template #main-body>
      <PageBody>
        <VCard>
          <div class="d-flex flex-row flex-no-wrap justify-space-between">
            <div class="flex-grow-1 my-4">
              <VCardTitle class="text-h4 text-wrap"> Status </VCardTitle>
              <VCardSubtitle class="text-wrap">
                <!-- {{ $t('pages.disaster_recovery.subtitle') }} -->
                Lorem ipsum dolor sit amet consectetur adipisicing elit. Voluptas et, dolorum
                dolorem sed, voluptatem officia dicta eveniet quibusdam nemo suscipit blanditiis vel
                ducimus. Corrupti, quasi totam. Deserunt officia impedit minus.
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
      </PageBody>
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
import { Principal } from '@dfinity/principal';
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
import PageBody from '~/components/layouts/PageBody.vue';
import PageHeader from '~/components/layouts/PageHeader.vue';
import PageLayout from '~/components/PageLayout.vue';
import { icAgent } from '~/core/ic-agent.core';
import { RegistryEntry, WasmModuleExtraChunks } from '~/generated/control-panel/control_panel.did';
import { GetDisasterRecoveryStateResponse } from '~/generated/upgrader/upgrader.did';
import { services } from '~/plugins/services.plugin';
import { UpgraderService } from '~/services/upgrader.service';
import { useStationStore } from '~/stores/station.store';
import { fetchCanisterIdlFromMetadata } from '~/utils/didc.utils';
import {
  blobToHumanReadable,
  DownloadedWasm,
  downloadRegistryEntry,
  drRequestArgs,
  stateToHumanReadable,
  systemInstallArgs,
} from '~/utils/disaster-recovery.utils';
import { focusText } from '~/utils/form.utils';
import { fetchCanisterControllers } from '~/utils/helper.utils';

const i18n = useI18n();

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
      wasm: DownloadedWasm | null;
    };

// type DisasterRecoveryStatus =
//   | {
//       type: 'typed';
//       data: GetDisasterRecoveryStateResponse;
//     }
//   | {
//       type: 'untyped';
//       data: object;
//     };

// function loadStatus(service: UpgraderService) : Promise<DisasterRecoveryStatus> {

// }

const state = ref<ConnectionState>({
  name: 'loading_upgrader',
  isLoading: true,
  error: '',
});

const stationIdlTextarea = ref<InstanceType<typeof VTextarea>>();

const humanReadableState = computed(() => {
  if (state.value.name === 'submitting_recovery') {
    return stateToHumanReadable(
      state.value.disasterRecoveryState as GetDisasterRecoveryStateResponse,
    );
  }

  return '';
});

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
  if (newSelectedRegistry) {
    const wasm = await downloadRegistryEntry(newSelectedRegistry as RegistryEntry);

    if (state.value.name === 'submitting_recovery') {
      state.value = {
        ...state.value,
        wasm: wasm,
      };
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

async function submitRecovery() {
  if (state.value.name === 'submitting_recovery' && drRequestPayload.value) {
    // state.value.submitLoading = true;
    const result = await upgraderService!.submitRecoveryUntyped(drRequestPayload.value);
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
    upgraderService = new UpgraderService(
      icAgent.get(),
      upgraderInfo.upgrader,
      upgraderInfo.candid,
    );
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
