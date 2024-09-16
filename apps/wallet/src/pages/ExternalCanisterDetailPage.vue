<template>
  <DataLoader
    :load="loadExternalCanister"
    @loading="loading = $event"
    @loaded="
      () => {
        // Load additional canister details, such as module hash and canister status.
        loadCanisterDetails();
        // Immediately after loading the canister with a query call,
        // we perform an update call to make sure the data is not tampered with.
        loadExternalCanister({
          verifiedCall: true,
        });
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
                    <div><VIcon :icon="mdiDatabase" size="x-small" /></div>
                  </VListItemTitle>
                </VListItem>
                <VListItem
                  :disabled="!canisterDetails.status.value"
                  @click="dialogs.icSettings = true"
                >
                  <VListItemTitle class="d-flex flex-nowrap ga-2">
                    <div class="flex-grow-1">{{ $t('external_canisters.ic_settings') }}</div>
                    <div><VIcon :icon="mdiInfinity" size="x-small" /></div>
                  </VListItemTitle>
                </VListItem>
                <VDivider />
                <VListItem @click="dialogs.unlink = true">
                  <VListItemTitle color="warning" class="d-flex flex-nowrap ga-2 text-error">
                    <div class="flex-grow-1">{{ $t('external_canisters.unlink') }}</div>
                    <div><VIcon :icon="mdiDatabaseOff" size="x-small" /></div>
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
          <template v-if="privileges.can_call.length" #actions>
            <VBtn size="default" color="primary" @click="dialogs.call = true">
              {{ $t('external_canisters.perform_call') }}
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
                <!-- TODO: Manage call policies -->
              </div>
              <VCard class="d-flex flex-column" :width="app.isMobile ? '100%' : '272px'">
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
                        <VBtn
                          v-if="privileges.can_change"
                          size="small"
                          density="compact"
                          color="default"
                          variant="tonal"
                          class="ml-1 px-2"
                          :append-icon="mdiDatabaseCog"
                          @click="dialogs.install = true"
                        >
                          {{ $t('external_canisters.install') }}
                        </VBtn>
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
                          <TextOverflow :max-length="24" :text="canisterDetails.moduleHash.value" />
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
                        {{ $t(`external_canisters.cycles`) }}
                        <template v-if="privileges.can_fund">
                          <CanisterTopUpDialog
                            v-model:open="dialogs.topUp"
                            :canister-id="canister.canister_id"
                          />

                          <VBtn
                            size="small"
                            density="compact"
                            color="default"
                            variant="tonal"
                            class="ml-1 px-2"
                            :append-icon="mdiDatabaseArrowUp"
                            @click="dialogs.topUp = true"
                          >
                            {{ $t('external_canisters.top_up') }}
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
                              toCyclesUnit(canisterDetails.status.value.cycles, CyclesUnit.Trillion)
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
  mdiContentCopy,
  mdiDatabase,
  mdiDatabaseArrowUp,
  mdiDatabaseCog,
  mdiDatabaseOff,
  mdiInfinity,
  mdiOpenInNew,
} from '@mdi/js';
import { Ref, computed, ref } from 'vue';
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
import CanisterSetupDialog from '~/components/external-canisters/CanisterSetupDialog.vue';
import CanisterTopUpDialog from '~/components/external-canisters/CanisterTopUpDialog.vue';
import PageBody from '~/components/layouts/PageBody.vue';
import PageHeader from '~/components/layouts/PageHeader.vue';
import RecentRequests from '~/components/requests/RecentRequests.vue';
import { Routes } from '~/configs/routes.config';
import { icAgent } from '~/core/ic-agent.core';
import logger from '~/core/logger.core';
import { ApiError } from '~/generated/control-panel/control_panel.did';
import {
  CanisterStatusResponse,
  ExternalCanister,
  ExternalCanisterCallerPrivileges,
} from '~/generated/station/station.did';
import { toCyclesUnit } from '~/mappers/cycles.mapper';
import { useAppStore } from '~/stores/app.store';
import { useStationStore } from '~/stores/station.store';
import { CyclesUnit, type PageProps } from '~/types/app.types';
import { Privilege } from '~/types/auth.types';
import { BreadCrumbItem } from '~/types/navigation.types';
import { RequestDomains } from '~/types/station.types';
import { copyToClipboard } from '~/utils/app.utils';
import { fetchCanisterModuleHash } from '~/utils/helper.utils';

const props = withDefaults(defineProps<PageProps>(), {
  title: undefined,
  breadcrumbs: () => [],
});
const router = useRouter();
const app = useAppStore();
const canister: Ref<ExternalCanister | null> = ref(null);
const canisterDetails = ref<{
  moduleHash: { value: string | null; loading: boolean };
  status: { value: CanisterStatusResponse | null; loading: boolean };
}>({
  moduleHash: { value: null, loading: false },
  status: { value: null, loading: false },
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
  call: false,
});

const privileges = ref<ExternalCanisterCallerPrivileges>(buildDefaultPrivileges());
const loading = ref(false);
const station = useStationStore();
const pageBreadcrumbs = computed<BreadCrumbItem[]>(() => {
  const breadcrumbs = [...props.breadcrumbs];

  if (canister.value) {
    breadcrumbs.push({
      title: canister.value.name,
    });
  }

  return breadcrumbs;
});

const loadExternalCanister = async (
  opts: {
    verifiedCall?: boolean;
  } = {},
): Promise<void> => {
  try {
    const canisterId = Principal.fromText(`${router.currentRoute.value.params.cid}`);
    const result = await station.service.getExternalCanisterByCanisterId(
      canisterId,
      opts?.verifiedCall,
    );

    canister.value = result.canister;
    privileges.value = result.privileges;
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

const loadCanisterDetails = (): void => {
  loadCanisterModuleHash();
  loadCanisterStatus();
};

const loadCanisterModuleHash = async (): Promise<void> => {
  try {
    if (!canister.value) {
      return;
    }

    canisterDetails.value.moduleHash.loading = true;

    canisterDetails.value.moduleHash.value = await fetchCanisterModuleHash(
      icAgent.get(),
      canister.value.canister_id,
    );
  } catch (err) {
    logger.error('Failed to load canister module hash', err);
  } finally {
    canisterDetails.value.moduleHash.loading = false;
  }
};

const loadCanisterStatus = async (): Promise<void> => {
  try {
    if (!canister.value) {
      return;
    }

    canisterDetails.value.status.loading = true;
    canisterDetails.value.status.value = await station.service.getExternalCanisterStatus(
      canister.value.canister_id,
    );
  } catch (err) {
    logger.error('Failed to load canister status', err);
  } finally {
    canisterDetails.value.status.loading = false;
  }
};
</script>
