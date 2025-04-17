<template>
  <PageLayout>
    <template #main-header>
      <PageHeader :title="$t('pages.disaster_recovery.title')" />
    </template>
    <template #main-body>
      <PageBody>
        <VCard color="info">
          <VCardText
            class="w-md-75 text-body-1 prose"
            v-html="$t('pages.disaster_recovery.help_block')"
          >
          </VCardText>
        </VCard>
        <VCard color="warning" class="mt-4">
          <VCardText
            class="w-md-75 text-body-1"
            v-html="$t('pages.disaster_recovery.warning_block')"
          >
          </VCardText>
        </VCard>
        <VCard
          style="max-width: 100%"
          :loading="upgraderState.name === 'loading_state'"
          class="mt-4"
        >
          <div class="d-flex flex-row flex-no-wrap justify-space-between" style="max-width: 100%">
            <div class="flex-grow-1 my-4" style="max-width: 100%">
              <VCardTitle class="text-h4 text-wrap">
                {{ $t('pages.disaster_recovery.upgrader_status') }}
              </VCardTitle>

              <template v-if="upgraderState.name === 'loading_upgrader'">
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

              <template v-else-if="upgraderState.name === 'upgrader_loaded'">
                <VCardText>
                  {{ $t('terms.upgrader') }}: {{ upgraderState.upgrader.upgrader.toText() }}
                </VCardText>

                <div
                  class="d-lg-flex flex-row flex-no-wrap justify-space-between"
                  style="max-width: 100%"
                >
                  <VCardText class="w-100 w-lg-50">
                    <VLabel>
                      {{ $t('pages.disaster_recovery.disaster_recovery_state') }}
                    </VLabel>
                    <VTextarea
                      v-if="upgraderState.disasterRecoveryState.name !== 'error'"
                      v-model="humanReadableState"
                      :rows="24"
                      class="font-monospace small-font no-wrap"
                      density="compact"
                      readonly
                    ></VTextarea>
                    <VAlert
                      v-else-if="upgraderState.disasterRecoveryState.name === 'error'"
                      color="error"
                    >
                      {{ upgraderState.disasterRecoveryState.error }}
                    </VAlert>
                  </VCardText>

                  <VCardText class="w-100 w-lg-50">
                    <VLabel>
                      {{ $t('pages.disaster_recovery.recent_logs') }}
                    </VLabel>
                    <VTextarea
                      v-if="upgraderState.logs.name === 'untyped'"
                      v-model="upgraderState.logs.candid"
                      density="compact"
                      :rows="24"
                      class="font-monospace small-font no-wrap"
                      readonly
                    ></VTextarea>

                    <VExpansionPanels
                      v-else-if="upgraderState.logs.name === 'typed'"
                      variant="accordion"
                    >
                      <VExpansionPanel v-for="log in upgraderState.logs.data.logs" :key="log.time">
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

                    <VAlert v-else-if="upgraderState.logs.name === 'error'" color="error">
                      {{ upgraderState.logs.error }}
                    </VAlert>
                  </VCardText>
                </div>
              </template>
              <template v-else-if="upgraderState.name === 'error'">
                <VCardText> {{ upgraderState.error }} </VCardText>
              </template>
            </div>
          </div>
        </VCard>
        <VCard class="mt-4" :loading="wasmPickingState.registryState.name === 'loading_registry'">
          <VCardTitle class="text-h4 text-wrap">
            {{ $t('pages.disaster_recovery.submit_recovery_request') }}
          </VCardTitle>
          <div class="d-flex flex-row flex-no-wrap justify-space-between">
            <div class="flex-grow-1 my-4">
              <template v-if="wasmPickingState.registryState.name === 'loaded_registry'">
                <VCardText>
                  <VSelect
                    v-if="wasmPickingState.registryState.name === 'loaded_registry'"
                    v-model="selectedRegistry"
                    :items="wasmPickingState.registryState.registry"
                    item-title="value.WasmModule.version"
                    item-value="id"
                    :placeholder="$t('pages.disaster_recovery.select_orbit_station_version')"
                    :return-object="true"
                    hide-details
                  />
                </VCardText>

                <div v-if="wasmPickingState.wasm?.wasmIdl">
                  <VCardText>
                    <VLabel>
                      {{ $t('pages.disaster_recovery.station_idl') }}
                    </VLabel>
                    <VTextarea
                      ref="stationIdlTextarea"
                      v-model="wasmPickingState.wasm.wasmIdl"
                      density="compact"
                      :rows="16"
                      class="font-monospace small-font no-wrap"
                      readonly
                    ></VTextarea>
                  </VCardText>
                </div>

                <VCardText>
                  <CanisterArgumentField
                    v-if="wasmPickingState.wasm"
                    ref="canisterArgumentField"
                    v-model="payload"
                    required
                    name="argument"
                    :rows="10"
                    :icon="false"
                    :candid="{
                      idl: wasmPickingState.wasm?.wasmIdl ?? '',
                      withType: { serviceParams: null },
                    }"
                  />
                </VCardText>

                <div
                  v-if="wasmPickingState.wasm?.wasmIdl"
                  class="d-flex flex-row flex-no-wrap justify-space-between"
                >
                  <VCardText v-if="upgraderState.name === 'upgrader_loaded'">
                    <VLabel>
                      {{ $t('pages.disaster_recovery.upgrader_idl') }}
                    </VLabel>
                    <VTextarea
                      ref="upgraderIdlTextarea"
                      v-model="upgraderState.upgrader.candid"
                      density="compact"
                      :rows="16"
                      class="font-monospace small-font no-wrap"
                      readonly
                    ></VTextarea>
                  </VCardText>
                  <VCardText>
                    <VLabel>
                      {{ $t('pages.disaster_recovery.station_service_payload') }}
                    </VLabel>
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
                    v-if="wasmPickingState.wasm && upgraderState.name === 'upgrader_loaded'"
                    ref="drRequestPayloadField"
                    v-model="drRequestPayload"
                    required
                    name="argument"
                    :candid="
                      upgraderState.name === 'upgrader_loaded'
                        ? {
                            idl: upgraderState.upgrader.candid,
                            withType: { methodParams: 'request_disaster_recovery' },
                          }
                        : undefined
                    "
                    :rows="10"
                    :icon="false"
                  />
                </VCardText>
              </template>

              <VCardActions>
                <VBtn
                  v-if="upgraderState.name === 'upgrader_loaded'"
                  color="primary"
                  variant="elevated"
                  size="large"
                  block
                  :loading="upgraderState.submitLoading"
                  :disabled="!drRequestPayload || upgraderState.submitLoading"
                  @click="submitRecovery"
                >
                  {{ $t('pages.disaster_recovery.submit_button') }}
                </VBtn>
              </VCardActions>

              <VCardText v-if="upgraderState.name === 'upgrader_loaded' && upgraderState.result">
                <VLabel>
                  {{ $t('pages.disaster_recovery.result') }}
                </VLabel>
                <VTextarea
                  v-model="upgraderState.result"
                  density="compact"
                  :rows="16"
                  class="font-monospace small-font no-wrap"
                  readonly
                ></VTextarea>
              </VCardText>
            </div>
          </div>
        </VCard>
      </PageBody>
    </template>
  </PageLayout>
</template>

<script setup lang="ts">
import { Principal } from '@dfinity/principal';
import { computed, markRaw, nextTick, onBeforeUnmount, onMounted, ref, watch } from 'vue';
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
import { useAppStore } from '~/stores/app.store';
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
const appStore = useAppStore();

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

useInterval(async () => {
  if (upgraderState.value.name === 'upgrader_loaded') {
    upgraderState.value.logs = await getLogs(
      upgraderState.value.upgraderService as UpgraderService,
    );
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
      upgraderService: UpgraderService;
      stateLoading: boolean;
      error: string;
    }
  | {
      name: 'upgrader_loaded';
      upgrader: UpgraderInfo;
      upgraderService: UpgraderService;

      disasterRecoveryState: DisasterRecoveryStateResult;
      logs: LogsResult;

      submitLoading: boolean;
      payload: Uint8Array;

      lastSubmitError: string | null;

      result: string | null | undefined;
    }
  | {
      name: 'error';
      error: string;
    };

type WasmPickingState = {
  registryState: RegistryState;
  wasm: DownloadedWasm | null;
};

const upgraderState = ref<ConnectionState>({
  name: 'loading_upgrader',
  isLoading: true,
  error: '',
});

const wasmPickingState = ref<WasmPickingState>({
  registryState: {
    name: 'loading_registry',
    isLoading: true,
    error: '',
  },
  wasm: null,
});

const stationIdlTextarea = ref<InstanceType<typeof VTextarea>>();
const upgraderIdlTextarea = ref<InstanceType<typeof VTextarea>>();

const humanReadableState = computed(() => {
  if (upgraderState.value.name === 'upgrader_loaded') {
    if (upgraderState.value.disasterRecoveryState.name === 'typed') {
      return stateToHumanReadable(
        upgraderState.value.disasterRecoveryState.data as GetDisasterRecoveryStateResponse,
      );
    } else if (upgraderState.value.disasterRecoveryState.name === 'untyped') {
      return upgraderState.value.disasterRecoveryState.candid;
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
      (upgraderState.value.name === 'upgrader_loaded'
        ? upgraderState.value.upgrader.upgrader
        : undefined) as Principal | undefined,
    () => wasmPickingState.value.wasm?.wasmIdl,
    () =>
      upgraderState.value.name === 'upgrader_loaded'
        ? upgraderState.value.disasterRecoveryState
        : undefined,
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

watch([payloadHumanReadable, () => wasmPickingState.value.wasm], ([payloadHumanReadable, wasm]) => {
  if (payloadHumanReadable && wasm) {
    drRequestPayloadField.value?.setArgument(
      drRequestArgs(payloadHumanReadable, wasm.extraChunks as WasmModuleExtraChunks),
    );
  }
});

watch(selectedRegistry, async newSelectedRegistry => {
  if (newSelectedRegistry) {
    const wasm = await downloadRegistryEntry(newSelectedRegistry as RegistryEntry);

    wasmPickingState.value = {
      ...wasmPickingState.value,
      wasm: wasm,
    };
    await nextTick();
    focusText(stationIdlTextarea.value?.$el.querySelector('textarea'), 'type SystemInstall');
    focusText(
      upgraderIdlTextarea.value?.$el.querySelector('textarea'),
      'type RequestDisasterRecoveryInput',
    );
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
  if (upgraderState.value.name === 'upgrader_loaded' && drRequestPayload.value) {
    upgraderState.value.submitLoading = true;
    try {
      const upgraderService = upgraderState.value.upgraderService;

      const result = await upgraderService.submitRecoveryUntyped(drRequestPayload.value);

      if (result == '(variant { Ok })') {
        wasmPickingState.value.wasm = null;
        selectedRegistry.value = null;
        appStore.sendNotification({
          type: 'success',
          message: i18n.t('pages.disaster_recovery.success_submit_recovery'),
        });
      } else {
        upgraderState.value.result = result;
      }

      upgraderState.value.logs = await getLogs(upgraderService as UpgraderService);
      upgraderState.value.disasterRecoveryState = await getDisasterRecoveryState(
        upgraderService as UpgraderService,
      );
    } catch (error) {
      upgraderState.value.lastSubmitError = i18n.t(
        'pages.disaster_recovery.error_submit_recovery',
        {
          error,
        },
      );
    } finally {
      upgraderState.value.submitLoading = false;
    }
  }
}

async function getLogs(service: UpgraderService): Promise<LogsResult> {
  try {
    const logs = await service.getLogs();

    return {
      name: 'typed',
      data: logs,
    };
  } catch (err: unknown) {
    if (err instanceof Object && 'code' in err && err.code === 'UNAUTHORIZED') {
      return {
        name: 'error',
        error: i18n.t('pages.disaster_recovery.error_logs_unauthorized'),
      };
    } else {
      try {
        const logs = await service.getLogsUntyped();
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

async function getDisasterRecoveryState(
  service: UpgraderService,
): Promise<DisasterRecoveryStateResult> {
  try {
    const drState = await service.getDisasterRecoveryState();
    return {
      name: 'typed',
      data: drState,
    };
  } catch (err: unknown) {
    if (err instanceof Object && 'code' in err && err.code === 'UNAUTHORIZED') {
      return {
        name: 'error',
        error: i18n.t('pages.disaster_recovery.error_state_unauthorized'),
      };
    } else {
      try {
        const drState = await service.getDisasterRecoveryStateUntyped();
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

async function initUpgraderState() {
  upgraderState.value = {
    name: 'loading_upgrader',
    isLoading: true,
    error: '',
  };

  try {
    const upgraderInfo = await getUpgrader();
    upgraderState.value = {
      name: 'loading_state',
      upgrader: upgraderInfo,
      upgraderService: markRaw(
        new UpgraderService(icAgent.get(), upgraderInfo.upgrader, upgraderInfo.candid),
      ),
      stateLoading: true,
      error: '',
    };
  } catch (error) {
    upgraderState.value = {
      name: 'error',
      error: i18n.t('pages.disaster_recovery.error_state', { error: error }),
    };
    return;
  } finally {
    if (unmounted.value) return;
  }

  const upgraderService = upgraderState.value.upgraderService as UpgraderService;

  const disasterRecoveryState = await getDisasterRecoveryState(upgraderService);
  const logs = await getLogs(upgraderService);

  if (unmounted.value) return;

  upgraderState.value = {
    name: 'upgrader_loaded',
    upgrader: upgraderState.value.upgrader,
    upgraderService,
    disasterRecoveryState,
    logs,
    submitLoading: false,
    lastSubmitError: null,
    payload: new Uint8Array(),
    result: null,
  };
}
async function initWasmPickingState() {
  try {
    const registryEntries = await controlPanelService.findRegistryEntries({
      filter_by: [{ Name: '@orbit/station' }],
      pagination: [],
      sort_by: [],
    });

    wasmPickingState.value = {
      registryState: {
        name: 'loaded_registry',
        registry: registryEntries.entries,
      },
      wasm: null,
    };
  } catch (error) {
    wasmPickingState.value = {
      registryState: {
        name: 'error',
        error: i18n.t('pages.disaster_recovery.error_registry_loading_failed', { error: error }),
      },
      wasm: null,
    };

    return;
  } finally {
    if (unmounted.value) return;
  }
}

onMounted(async () => {
  Promise.all([initUpgraderState(), initWasmPickingState()]);
});

onBeforeUnmount(() => {
  unmounted.value = true;
});
</script>

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
.prose {
  p,
  ol {
    margin-bottom: 1rem;
  }
  ol {
    margin-left: 2rem;
  }
}
</style>
