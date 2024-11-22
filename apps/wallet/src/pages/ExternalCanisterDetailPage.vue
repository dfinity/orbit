<template>
  <DataLoader
    v-model:force-reload="forceReload"
    :load="loadExternalCanister"
    :refresh-interval-ms="5000"
    :disable-refresh="disableRefresh"
    @loaded="
      () => {
        loading = false;
        // Immediately after loading the canister with a query call, we mark the page to be verified next,
        // this way all subsequent calls will be verified.
        verifiedPageLoad = true;
      }
    "
    @failed="
      () => {
        loading = false;
      }
    "
  >
    <template #error>
      <PageLayout>
        <template #main-header>
          <PageHeader
            :title="$t('external_canisters.loading_error')"
            :breadcrumbs="pageBreadcrumbs"
          />
        </template>
      </PageLayout>
    </template>
    <PageLayout>
      <template #main-header>
        <div v-if="loading" class="d-flex justify-center">
          <VProgressCircular indeterminate color="primary" class="ma-8" />
        </div>
        <div v-else-if="!canister">
          <PageHeader :title="$t('external_canisters.not_found')" :breadcrumbs="pageBreadcrumbs" />
        </div>
        <PageHeader v-else :title="pageTitle" :breadcrumbs="pageBreadcrumbs">
          <template #title-toolbar>
            <template v-if="privileges.can_change">
              <CanisterSetupDialog
                v-model:open="dialogs.settings"
                :canister-id="canister.canister_id"
              />
              <CanisterUnlinkDialog
                v-model:open="dialogs.unlink"
                :canister-id="canister.canister_id"
              />
              <CanisterIcSettingsDialog
                v-if="canisterDetails.status.value"
                v-model:open="dialogs.icSettings"
                :canister-id="canister.canister_id"
                :canister-settings="{
                  compute_allocation: canisterDetails.status.value.settings.compute_allocation,
                  controllers: canisterDetails.status.value.settings.controllers as Principal[],
                  freezing_threshold: canisterDetails.status.value.settings.freezing_threshold,
                  memory_allocation: canisterDetails.status.value.settings.memory_allocation,
                  reserved_cycles_limit:
                    canisterDetails.status.value.settings.reserved_cycles_limit,
                  wasm_memory_limit: canisterDetails.status.value.settings.wasm_memory_limit,
                  log_visibility: canisterDetails.status.value.settings.log_visibility,
                }"
              />
            </template>
            <VMenu v-if="privileges.can_change">
              <template #activator="{ props: menuProps }">
                <VBtn
                  class="px-1 mb-2"
                  size="small"
                  color="default"
                  variant="tonal"
                  :text="$t('terms.settings')"
                  v-bind="menuProps"
                />
              </template>
              <VList density="compact">
                <VListItem @click="dialogs.settings = true">
                  <VListItemTitle class="d-flex flex-nowrap ga-2">
                    <div class="flex-grow-1">{{ $t('external_canisters.configuration') }}</div>
                    <div>
                      <VIcon :icon="mdiDatabase" size="x-small" />
                    </div>
                  </VListItemTitle>
                </VListItem>
                <VListItem
                  :disabled="!canisterDetails.status.value"
                  @click="dialogs.icSettings = true"
                >
                  <VListItemTitle class="d-flex flex-nowrap ga-2">
                    <div class="flex-grow-1">{{ $t('external_canisters.ic_settings') }}</div>
                    <div>
                      <VIcon :icon="mdiInfinity" size="x-small" />
                    </div>
                  </VListItemTitle>
                </VListItem>
                <VDivider />
                <VListItem @click="dialogs.unlink = true">
                  <VListItemTitle color="warning" class="d-flex flex-nowrap ga-2 text-error">
                    <div class="flex-grow-1">{{ $t('external_canisters.unlink') }}</div>
                    <div>
                      <VIcon :icon="mdiDatabaseOff" size="x-small" />
                    </div>
                  </VListItemTitle>
                </VListItem>
              </VList>
            </VMenu>
            <BtnCanisterSetup
              v-else
              :canister-id="canister.canister_id"
              class="px-1 mb-2"
              size="small"
              color="default"
              variant="tonal"
              :readonly="!privileges.can_change"
              :text="$t('terms.settings')"
            />
          </template>
          <template #subtitle>
            <div class="d-flex flex-column ga-2">
              <div class="d-flex flex-row align-center">
                <small>
                  <TextOverflow :max-length="32" :text="canister.canister_id.toText()" />
                </small>
                <VBtn
                  size="x-small"
                  variant="text"
                  :icon="mdiContentCopy"
                  @click="
                    copyToClipboard({
                      textToCopy: canister.canister_id.toText(),
                      sendNotification: true,
                    })
                  "
                />
              </div>
            </div>
          </template>
          <template
            v-if="
              privileges.can_call.length ||
              hasRequiredPrivilege({ anyOf: [Privilege.CallAnyExternalCanister] })
            "
            #actions
          >
            <CanisterCallDialog
              :open="dialogs.call"
              :canister-id="canister.canister_id"
              :allowed-methods="mapAllowedCanisterMethods(privileges.can_call)"
              :allow-any-method="
                hasRequiredPrivilege({ anyOf: [Privilege.CallAnyExternalCanister] })
              "
              :canister-candid-idl="
                canisterDetails.candid.value !== null ? canisterDetails.candid.value.idl : undefined
              "
              @update:open="dialogs.call = $event"
            />
            <VBtn
              size="default"
              color="primary"
              :disabled="!canisterDetails.moduleHash.value"
              @click="dialogs.call = true"
            >
              {{ $t('external_canisters.perform_call.title') }}
            </VBtn>
          </template>
        </PageHeader>
      </template>
      <template v-if="!loading" #main-body>
        <PageBody v-if="!canister">{{ $t('external_canisters.not_found_description') }}</PageBody>
        <PageBody v-else>
          <AuthCheck :privileges="[Privilege.ListRequests]">
            <RecentRequests
              class="mb-4"
              :see-all-link="{
                name: Routes.Requests,
                query: {
                  group_by: RequestDomains.ExternalCanisters,
                  canister_id: canister.canister_id.toText(),
                },
              }"
              :types="[
                { ConfigureExternalCanister: [canister.canister_id] },
                { FundExternalCanister: [canister.canister_id] },
                { ChangeExternalCanister: [canister.canister_id] },
                { CallExternalCanister: [canister.canister_id] },
              ]"
              hide-not-found
            />
          </AuthCheck>
          <VRow>
            <VCol
              cols="12"
              class="d-flex flex-column-reverse flex-md-row align-md-start flex-no-wrap ga-4"
            >
              <div class="d-flex flex-column flex-grow-1 ga-4 align-self-stretch">
                <CanisterConfigureMethodCallList
                  :canister-id="canister.canister_id"
                  :request-policies="canister.request_policies.calls"
                  :permissions="canister.permissions.calls"
                  :readonly="!privileges.can_change"
                  :canister-candid-idl="
                    canisterDetails.candid.value !== null
                      ? canisterDetails.candid.value.idl
                      : undefined
                  "
                  @editing="disableRefresh = $event"
                />
              </div>
              <div
                :style="{ 'min-width': app.isMobile ? '100%' : '272px' }"
                class="d-flex flex-column ga-4"
              >
                <VCard class="d-flex flex-column" width="100%">
                  <VToolbar color="transparent" class="pr-4">
                    <VToolbarTitle>
                      {{ $t('terms.canister') }}
                      <VBtn
                        v-if="canister"
                        size="x-small"
                        variant="text"
                        :icon="mdiOpenInNew"
                        density="comfortable"
                        class="ml-1"
                        :href="`https://dashboard.internetcomputer.org/canister/${canister.canister_id.toText()}`"
                        target="_blank"
                      />
                    </VToolbarTitle>
                    <VIcon :icon="mdiDatabase" />
                  </VToolbar>
                  <VCardText class="pt-0 d-flex flex-column flex-grow-1">
                    <VList lines="two" class="bg-transparent pt-0">
                      <VListItem class="pt-0 px-0">
                        <VListItemTitle class="font-weight-bold">
                          {{ $t(`external_canisters.module_hash`) }}
                          <template v-if="privileges.can_change">
                            <CanisterInstallDialog
                              :key="canisterDetails.moduleHash.value?.toString()"
                              v-model:open="dialogs.install"
                              :canister-id="canister.canister_id"
                              :canister-module-hash="
                                canisterDetails.moduleHash.value !== null
                                  ? canisterDetails.moduleHash.value
                                  : undefined
                              "
                              :canister-candid-idl="
                                canisterDetails.candid.value !== null
                                  ? canisterDetails.candid.value.idl
                                  : undefined
                              "
                            />
                            <VBtn
                              size="small"
                              density="compact"
                              color="default"
                              variant="tonal"
                              class="ml-1 px-2"
                              :disabled="!canisterDetails.status.value"
                              :append-icon="mdiDatabaseCog"
                              @click="dialogs.install = true"
                            >
                              {{ $t('external_canisters.install') }}
                            </VBtn>
                          </template>
                        </VListItemTitle>
                        <VListItemSubtitle>
                          <VProgressCircular
                            v-if="canisterDetails.moduleHash.loading"
                            indeterminate
                            color="primary"
                            class="mt-2"
                            size="16"
                          />
                          <span v-else-if="!canisterDetails.moduleHash.value">
                            {{ $t('terms.none') }}
                          </span>
                          <span v-else>
                            <TextOverflow
                              :max-length="24"
                              :text="canisterDetails.moduleHash.value"
                            />
                            <VBtn
                              size="small"
                              variant="text"
                              :icon="mdiContentCopy"
                              @click="
                                copyToClipboard({
                                  textToCopy: canisterDetails.moduleHash.value,
                                  sendNotification: true,
                                })
                              "
                            />
                          </span>
                        </VListItemSubtitle>
                      </VListItem>
                      <VListItem class="pt-0 px-0">
                        <VListItemTitle class="font-weight-bold">
                          <VIcon
                            v-if="canister.monitoring.length"
                            :icon="mdiBatteryChargingMedium"
                            :tooltip="$t(`external_canisters.cycles`)"
                          />
                          {{ $t(`external_canisters.cycles`) }}
                          <template v-if="privileges.can_fund">
                            <CanisterTopUpDialog
                              v-model:open="dialogs.topUp"
                              :canister-id="canister.canister_id"
                            />
                            <CanisterMonitorDialog
                              v-model:open="dialogs.monitor"
                              :canister-id="canister.canister_id"
                            />

                            <VBtn
                              size="small"
                              density="compact"
                              color="default"
                              variant="tonal"
                              class="ml-1 px-2"
                              :append-icon="mdiBatteryArrowUpOutline"
                              @click="dialogs.topUp = true"
                            >
                              {{ $t('external_canisters.top_up') }}
                            </VBtn>

                            <VBtn
                              v-if="!canister.monitoring.length"
                              :disabled="canisterDetails.status.loading"
                              size="small"
                              density="compact"
                              color="default"
                              variant="tonal"
                              class="ml-1 px-2"
                              :append-icon="mdiBatterySyncOutline"
                              @click="dialogs.monitor = true"
                            >
                              {{ $t('external_canisters.monitor.title') }}
                            </VBtn>
                            <VBtn
                              v-if="canister.monitoring.length"
                              :disabled="canisterDetails.status.loading"
                              size="small"
                              density="compact"
                              color="default"
                              variant="tonal"
                              class="ml-1 px-2"
                              :append-icon="mdiBatteryOffOutline"
                              @click="removeMonitoring"
                            >
                              {{ $t('external_canisters.monitor.stop_title') }}
                            </VBtn>
                          </template>
                        </VListItemTitle>
                        <VListItemSubtitle>
                          <VProgressCircular
                            v-if="canisterDetails.status.loading"
                            indeterminate
                            color="primary"
                            class="mt-2"
                            size="16"
                          />
                          <span v-else-if="canisterDetails.status.value == null">
                            {{ $t('external_canisters.not_controller') }}
                          </span>
                          <span v-else>
                            <template
                              v-if="
                                toCyclesUnit(
                                  canisterDetails.status.value.cycles,
                                  CyclesUnit.Trillion,
                                ) !== 0
                              "
                            >
                              {{
                                toCyclesUnit(
                                  canisterDetails.status.value.cycles,
                                  CyclesUnit.Trillion,
                                )
                              }}
                              {{ $t('cycles.units.tc') }}
                            </template>
                            <template v-else>
                              {{ canisterDetails.status.value.cycles }}
                              {{ $t('cycles.units.e8s') }}
                            </template>
                          </span>
                        </VListItemSubtitle>
                      </VListItem>
                    </VList>
                  </VCardText>
                </VCard>

                <AuthCheck :privileges="[Privilege.ListRequests]">
                  <RecentRequests
                    class="mb-4"
                    :title="$t('external_canisters.performed_calls')"
                    :show-items-title="false"
                    :see-all-link="{
                      name: Routes.Requests,
                      query: {
                        group_by: RequestDomains.ExternalCanisters,
                        canister_id: canister.canister_id.toText(),
                        statuses: ['Processing', 'Completed', 'Failed'],
                      },
                    }"
                    :sort-by="{ lastModified: 'desc' }"
                    :limit="1"
                    :statuses="[{ Processing: null }, { Completed: null }, { Failed: null }]"
                    :types="[{ CallExternalCanister: [canister.canister_id] }]"
                    hide-not-found
                  />
                </AuthCheck>
              </div>
            </VCol>
          </VRow>
        </PageBody>
      </template>
    </PageLayout>
  </DataLoader>
</template>

<script lang="ts" setup>
import { Principal } from '@dfinity/principal';
import {
  mdiBatteryArrowUpOutline,
  mdiBatteryChargingMedium,
  mdiBatteryOffOutline,
  mdiBatterySyncOutline,
  mdiContentCopy,
  mdiDatabase,
  mdiDatabaseCog,
  mdiDatabaseOff,
  mdiInfinity,
  mdiOpenInNew,
} from '@mdi/js';
import { Ref, computed, ref, watch } from 'vue';
import { useRouter } from 'vue-router';
import {
  VBtn,
  VCard,
  VCardText,
  VCol,
  VDivider,
  VIcon,
  VList,
  VListItem,
  VListItemSubtitle,
  VListItemTitle,
  VMenu,
  VProgressCircular,
  VRow,
  VToolbar,
  VToolbarTitle,
} from 'vuetify/components';
import AuthCheck from '~/components/AuthCheck.vue';
import DataLoader from '~/components/DataLoader.vue';
import PageLayout from '~/components/PageLayout.vue';
import TextOverflow from '~/components/TextOverflow.vue';
import BtnCanisterSetup from '~/components/external-canisters/BtnCanisterSetup.vue';
import CanisterCallDialog from '~/components/external-canisters/CanisterCallDialog.vue';
import CanisterConfigureMethodCallList from '~/components/external-canisters/CanisterConfigureMethodCallList.vue';
import CanisterIcSettingsDialog from '~/components/external-canisters/CanisterIcSettingsDialog.vue';
import CanisterInstallDialog from '~/components/external-canisters/CanisterInstallDialog.vue';
import CanisterMonitorDialog from '~/components/external-canisters/CanisterMonitorDialog.vue';
import CanisterSetupDialog from '~/components/external-canisters/CanisterSetupDialog.vue';
import CanisterTopUpDialog from '~/components/external-canisters/CanisterTopUpDialog.vue';
import CanisterUnlinkDialog from '~/components/external-canisters/CanisterUnlinkDialog.vue';
import PageBody from '~/components/layouts/PageBody.vue';
import PageHeader from '~/components/layouts/PageHeader.vue';
import RecentRequests from '~/components/requests/RecentRequests.vue';
import {
  useExternalCanisterProvider,
  useLoadExternalCanisterModuleHash,
  useLoadExternalCanisterStatus,
} from '~/composables/external-canisters.composable';
import { Routes } from '~/configs/routes.config';
import logger from '~/core/logger.core';
import { ApiError } from '~/generated/control-panel/control_panel.did';
import {
  CanisterStatusResponse,
  ExternalCanister,
  ExternalCanisterCallerPrivileges,
} from '~/generated/station/station.did';
import { toCyclesUnit } from '~/mappers/cycles.mapper';
import { mapAllowedCanisterMethods } from '~/mappers/external-canister.mapper';
import { useAppStore } from '~/stores/app.store';
import { useStationStore } from '~/stores/station.store';
import { CyclesUnit, type PageProps } from '~/types/app.types';
import { Privilege } from '~/types/auth.types';
import { BreadCrumbItem } from '~/types/navigation.types';
import { RequestDomains } from '~/types/station.types';
import { copyToClipboard } from '~/utils/app.utils';
import { hasRequiredPrivilege } from '~/utils/auth.utils';
import { fetchCanisterIdlFromMetadata } from '~/utils/didc.utils';
import { assertAndReturn, debounce } from '~/utils/helper.utils';
import {
  useOnFailedOperation,
  useOnSuccessfulOperation,
} from '~/composables/notifications.composable.ts';

const props = withDefaults(defineProps<PageProps>(), {
  title: undefined,
  breadcrumbs: () => [],
});
const router = useRouter();
const app = useAppStore();
const canister: Ref<ExternalCanister | null> = ref(null);
const canisterDetails = ref<{
  moduleHash: { value: string | null; loading: boolean; initialized: boolean };
  status: { value: CanisterStatusResponse | null; loading: boolean; initialized: boolean };
  candid: {
    value: { idl: string } | null;
    loading: boolean;
    initialized: boolean;
  };
}>({
  moduleHash: { value: null, loading: false, initialized: false },
  status: { value: null, loading: false, initialized: false },
  candid: { value: null, loading: false, initialized: false },
});

const pageTitle = computed(() => {
  if (props.title) {
    return props.title;
  }

  return canister.value?.name ?? '';
});

const buildDefaultPrivileges = (): ExternalCanisterCallerPrivileges => ({
  canister_id: Principal.anonymous(),
  id: '',
  can_change: false,
  can_fund: false,
  can_call: [],
});

const dialogs = ref({
  settings: false,
  unlink: false,
  icSettings: false,
  install: false,
  topUp: false,
  monitor: false,
  call: false,
});

const { register } = useExternalCanisterProvider();
const currentRouteCanisterId = computed(() => `${router.currentRoute.value.params.cid}`);
const verifiedPageLoad = ref(false);
const privileges = ref(buildDefaultPrivileges()) as Ref<ExternalCanisterCallerPrivileges>;
const loading = ref(false);
const station = useStationStore();
const disableRefresh = ref(false);
const forceReload = ref(false);
const pageBreadcrumbs = computed<BreadCrumbItem[]>(() => {
  const breadcrumbs = [...props.breadcrumbs];

  if (canister.value) {
    breadcrumbs.push({
      title: canister.value.name,
    });
  }

  return breadcrumbs;
});

watch(
  dialogs,
  () => {
    // Disable refresh when any dialog is open.
    disableRefresh.value = Object.values(dialogs.value).some(open => open);
  },
  { deep: true },
);

watch(
  currentRouteCanisterId,
  (current, previous) => {
    if (current !== previous) {
      // Reset the page state when the canister id changes.
      loading.value = true;
      verifiedPageLoad.value = false;
      canisterDetails.value = {
        moduleHash: { value: null, loading: false, initialized: false },
        status: { value: null, loading: false, initialized: false },
        candid: { value: null, loading: false, initialized: false },
      };
      privileges.value = buildDefaultPrivileges();
      canister.value = null;

      // Register the canister id for other components to use.
      register(current);
    }
  },
  { immediate: true },
);

const loadExternalCanisterStatus = debounce(
  (canisterId: Principal) => {
    canisterDetails.value.status.loading = !canisterDetails.value.status.initialized;
    useLoadExternalCanisterStatus(canisterId)
      .then(status => {
        canisterDetails.value.status.value = status;
      })
      .catch(err => {
        logger.error('Failed to load external canister status', err);
      })
      .finally(() => {
        canisterDetails.value.status.loading = false;
        canisterDetails.value.status.initialized = true;
      });
  },
  20_000, // A status call is a call through the station canister, to prevent high load we make less frequent calls.
  { immediate: true },
);

const loadExternalCanisterModuleHash = debounce(
  (canisterId: Principal) => {
    canisterDetails.value.moduleHash.loading = !canisterDetails.value.moduleHash.initialized;
    useLoadExternalCanisterModuleHash(canisterId)
      .then(moduleHash => {
        canisterDetails.value.moduleHash.value = moduleHash;
      })
      .catch(err => {
        logger.error('Failed to load external canister module hash', err);
      })
      .finally(() => {
        canisterDetails.value.moduleHash.loading = false;
        canisterDetails.value.moduleHash.initialized = true;
      });
  },
  2_000, // Module hash loading is cheap and can be done with a readState call to the IC.
  { immediate: true },
);

const loadExternalCanisterCandidIdl = debounce(
  (canisterId: Principal) => {
    canisterDetails.value.candid.loading = !canisterDetails.value.candid.initialized;
    fetchCanisterIdlFromMetadata(canisterId)
      .then(idl => {
        canisterDetails.value.candid.value = idl ? { idl } : null;
      })
      .catch(err => {
        logger.error('Failed to read canister candid interface', err);
      })
      .finally(() => {
        canisterDetails.value.candid.loading = false;
        canisterDetails.value.candid.initialized = true;
      });
  },
  2_000, // Module hash loading is cheap and can be done with a readState call to the IC.
  { immediate: true },
);

const loadExternalCanister = async (): Promise<void> => {
  try {
    const canisterId = Principal.fromText(currentRouteCanisterId.value);
    const result = await station.service.getExternalCanisterByCanisterId(
      canisterId,
      verifiedPageLoad.value,
    );

    if (disableRefresh.value) {
      // If the page is disabled for refresh, we don't update the canister and privileges.
      return;
    }

    canister.value = result.canister;
    privileges.value = result.privileges;

    // Load additional canister details, such as module hash and canister status.
    loadExternalCanisterModuleHash(result.canister.canister_id);
    loadExternalCanisterStatus(result.canister.canister_id);
    loadExternalCanisterCandidIdl(result.canister.canister_id);
  } catch (err) {
    const error = err as ApiError;

    if (error?.code && error.code === 'INVALID_EXTERNAL_CANISTER') {
      canister.value = null;
      privileges.value = buildDefaultPrivileges();

      return;
    }

    logger.error('Failed to load external canister', error);
  }
};

const removeMonitoring = async (): Promise<void> => {
  try {
    canisterDetails.value.status.loading = true;

    const request = await station.service.monitorExternalCanister({
      canister_id: assertAndReturn(Principal.fromText(currentRouteCanisterId.value), 'canisterId'),
      kind: {
        Stop: null,
      },
    });

    useOnSuccessfulOperation(request);
  } catch (error) {
    logger.error('Failed to submit monitoring request', error);

    useOnFailedOperation();
  } finally {
    canisterDetails.value.status.loading = false;
  }
};
</script>
