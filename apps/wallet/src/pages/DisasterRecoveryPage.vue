<template>
  <PageLayout>
    <template #main-header>
      <PageHeader :title="$t('pages.disaster_recovery.title')" />
    </template>
    <template #main-body>
      <PageBody>
        <VCard style="max-width: 100%">
          <div class="d-flex flex-row flex-no-wrap justify-space-between" style="max-width: 100%">
            <div class="flex-grow-1 my-4" style="max-width: 100%">
              <VCardTitle class="text-h4 text-wrap"> Status </VCardTitle>

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

              <template v-if="state.name === 'submitting_recovery'">
                <VCardText> Upgrader: {{ state.upgrader.upgrader.toText() }} </VCardText>

                <div
                  class="d-lg-flex flex-row flex-no-wrap justify-space-between"
                  style="max-width: 100%"
                >
                  <VCardText class="w-100 w-lg-50">
                    <VLabel>Disaster Recovery State</VLabel>
                    <VTextarea
                      :rows="24"
                      class="font-monospace small-font no-wrap"
                      density="compact"
                      v-model="humanReadableState"
                      readonly
                    ></VTextarea>
                  </VCardText>

                  <VCardText class="w-100 w-lg-50">
                    <VLabel>Recent Logs</VLabel>
                    <VTextarea
                      v-if="state.logs.name === 'untyped'"
                      v-model="state.logs.candid"
                      density="compact"
                      :rows="24"
                      class="font-monospace small-font no-wrap"
                      readonly
                    ></VTextarea>

                    <VExpansionPanels variant="accordion" v-else-if="state.logs.name === 'typed'">
                      <VExpansionPanel v-for="log in state.logs.data.logs" :key="log.time">
                        <VExpansionPanelTitle class="d-flex flex-row flex-no-wrap">
                          <div style="display: flex; flex: 1; overflow: hidden">
                            <div style="width: 200px; flex-shrink: 0">
                              {{ getLogTime(log) }}
                            </div>
                            <div class="no-wrap-ellipsis">
                              {{ log.message }}
                            </div>
                          </div>
                        </VExpansionPanelTitle>
                        <VExpansionPanelText>
                          <div class="">{{ log.message }}</div>
                          <div class="font-monospace small-font mt-4">{{ log.data_json }}</div>
                        </VExpansionPanelText>
                      </VExpansionPanel>
                    </VExpansionPanels>
                  </VCardText>
                </div>
              </template>
            </div>
          </div>
        </VCard>
        <VCard class="mt-4">
          <VCardTitle class="text-h4 text-wrap"> Submit Recovery Request </VCardTitle>
          <div class="d-flex flex-row flex-no-wrap justify-space-between">
            <div class="flex-grow-1 my-4">
              <template v-if="state.name === 'submitting_recovery'">
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

                <div v-if="state.wasm?.wasmIdl">
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
                    :icon="false"
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
                    :icon="false"
                  />
                </VCardText>
              </template>

              <VCardActions>
                <VBtn
                  v-if="state.name === 'submitting_recovery'"
                  color="primary"
                  variant="elevated"
                  size="large"
                  block
                  :loading="state.submitLoading"
                  :disabled="!drRequestPayload || state.submitLoading"
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
.no-wrap-ellipsis {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
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
  VCardText,
  VCardTitle,
  VProgressCircular,
  VTextarea,
} from 'vuetify/components';
import CanisterArgumentField from '~/components/inputs/CanisterArgumentField.vue';
import PageBody from '~/components/layouts/PageBody.vue';
import PageHeader from '~/components/layouts/PageHeader.vue';
import PageLayout from '~/components/PageLayout.vue';
import { useInterval } from '~/composables/util.composable';
import { icAgent } from '~/core/ic-agent.core';
import { RegistryEntry, WasmModuleExtraChunks } from '~/generated/control-panel/control_panel.did';
import {
  GetDisasterRecoveryStateResponse,
  GetLogsResponse,
  LogEntry,
} from '~/generated/upgrader/upgrader.did';
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

useInterval(async () => {
  if (state.value.name === 'submitting_recovery') {
    state.value.logs = await getLogs();
  }
}, 5000);

type RegistryState =
  | {
      name: 'loading_registry';
      isLoading: boolean;
      error: string;
    }
  | {
      name: 'loaded_registry';
      registry: RegistryEntry[];
    }
  | {
      name: 'error';
      error: string;
    };

type DisasterRecoveryStateResult =
  | {
      name: 'typed';
      data: GetDisasterRecoveryStateResponse;
    }
  | {
      name: 'untyped';
      candid: string;
    }
  | {
      name: 'error';
      error: string;
    };

type LogsResult =
  | {
      name: 'typed';
      data: GetLogsResponse;
    }
  | {
      name: 'untyped';
      candid: string;
    }
  | {
      name: 'error';
      error: string;
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
      stateLoading: boolean;
      error: string;
    }
  | {
      name: 'submitting_recovery';
      upgrader: UpgraderInfo;
      disasterRecoveryState: DisasterRecoveryStateResult;
      logs: LogsResult;
      registryState: RegistryState;
      submitLoading: boolean;
      error: string;
      payload: Uint8Array;
      wasm: DownloadedWasm | null;
    };

const state = ref<ConnectionState>({
  name: 'loading_upgrader',
  isLoading: true,
  error: '',
});

const stationIdlTextarea = ref<InstanceType<typeof VTextarea>>();

const humanReadableState = computed(() => {
  if (state.value.name === 'submitting_recovery') {
    if (state.value.disasterRecoveryState.name === 'typed') {
      return stateToHumanReadable(
        state.value.disasterRecoveryState.data as GetDisasterRecoveryStateResponse,
      );
    } else if (state.value.disasterRecoveryState.name === 'untyped') {
      return state.value.disasterRecoveryState.candid;
    } else {
      return '';
    }
  }
});

function getLogTime(log: LogEntry) {
  const time = new Date(log.time);
  return time.toLocaleDateString() + ' ' + time.toLocaleTimeString();
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
      if (disasterRecoveryState.name === 'typed') {
        canisterArgumentField.setArgument(
          systemInstallArgs(
            upgraderId,
            disasterRecoveryState.data as GetDisasterRecoveryStateResponse,
          ),
        );
      }
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
    state.value.submitLoading = true;
    try {
      await upgraderService!.submitRecoveryUntyped(drRequestPayload.value);

      state.value.wasm = null;
      selectedRegistry.value = null;

      state.value.logs = await getLogs();
      state.value.disasterRecoveryState = await getDisasterRecoveryState();
    } catch (error) {
      state.value.error = i18n.t('pages.disaster_recovery.error_submit_recovery', { error });
    } finally {
      state.value.submitLoading = false;
    }
  }
}

async function getLogs(): Promise<LogsResult> {
  try {
    const logs = await upgraderService!.getLogs();

    return {
      name: 'typed',
      data: logs,
    };
  } catch (err: unknown) {
    if (err instanceof Object && 'code' in err && err.code === 'UNAUTHORIZED') {
      return {
        name: 'error',
        error: i18n.t('pages.disaster_recovery.error_unauthorized'),
      };
    } else {
      try {
        const logs = await upgraderService!.getLogsUntyped();
        return {
          name: 'untyped',
          candid: logs,
        };
      } catch (err: unknown) {
        return {
          name: 'error',
          error: i18n.t('pages.disaster_recovery.error_logs_loading_failed', { error: err }),
        };
      }
    }
  }
}

async function getDisasterRecoveryState(): Promise<DisasterRecoveryStateResult> {
  try {
    const drState = await upgraderService!.getDisasterRecoveryState();
    return {
      name: 'typed',
      data: drState,
    };
  } catch (err: unknown) {
    if (err instanceof Object && 'code' in err && err.code === 'UNAUTHORIZED') {
      return {
        name: 'error',
        error: i18n.t('pages.disaster_recovery.error_unauthorized'),
      };
    } else {
      try {
        const drState = await upgraderService!.getDisasterRecoveryStateUntyped();
        return {
          name: 'untyped',
          candid: drState,
        };
      } catch (err: unknown) {
        return {
          name: 'error',
          error: i18n.t('pages.disaster_recovery.error_state_loading_failed', { error: err }),
        };
      }
    }
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
    const drState = await getDisasterRecoveryState();

    state.value = {
      name: 'submitting_recovery',
      upgrader: state.value.upgrader,
      disasterRecoveryState: drState,
      logs: await getLogs(),
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
    console.log(state.value.logs);
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
