<template>
  <DataLoader
    :load="loadExternalCanister"
    @loading="loading = $event"
    @loaded="
      () => {
        loadExternalCanister({
          // Immediately after loading the canister with a query call,
          // we perform an update call to make sure the data is not tampered with.
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
                :dialog-max-width="800"
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
                <VDivider />
                <VListItem @click="dialogs.unlink = true">
                  <VListItemTitle color="warning" class="d-flex flex-nowrap ga-2 text-error">
                    <div class="flex-grow-1">{{ $t('external_canisters.unlink') }}</div>
                    <div><VIcon :icon="mdiDatabaseEyeOff" size="x-small" /></div>
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
          <template v-if="privileges.can_fund" #actions>
            <VBtn size="default" color="default" variant="outlined" :append-icon="mdiInfinity">
              {{ $t('external_canisters.ic_settings') }}
            </VBtn>
            <VBtn v-if="privileges.can_fund" size="default" color="primary">
              {{ $t('external_canisters.add_cycles') }}
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
        </PageBody>
      </template>
    </PageLayout>
  </DataLoader>
</template>

<script lang="ts" setup>
import { Principal } from '@dfinity/principal';
import { mdiContentCopy, mdiDatabase, mdiDatabaseEyeOff, mdiInfinity } from '@mdi/js';
import { Ref, computed, ref } from 'vue';
import { useRouter } from 'vue-router';
import {
  VBtn,
  VDivider,
  VIcon,
  VList,
  VListItem,
  VListItemTitle,
  VMenu,
  VProgressCircular,
} from 'vuetify/components';
import AuthCheck from '~/components/AuthCheck.vue';
import DataLoader from '~/components/DataLoader.vue';
import PageLayout from '~/components/PageLayout.vue';
import TextOverflow from '~/components/TextOverflow.vue';
import BtnCanisterSetup from '~/components/external-canisters/BtnCanisterSetup.vue';
import CanisterSetupDialog from '~/components/external-canisters/CanisterSetupDialog.vue';
import PageBody from '~/components/layouts/PageBody.vue';
import PageHeader from '~/components/layouts/PageHeader.vue';
import RecentRequests from '~/components/requests/RecentRequests.vue';
import { Routes } from '~/configs/routes.config';
import logger from '~/core/logger.core';
import { ApiError } from '~/generated/control-panel/control_panel.did';
import {
  ExternalCanister,
  ExternalCanisterCallerPrivileges,
} from '~/generated/station/station.did';
import { useStationStore } from '~/stores/station.store';
import type { PageProps } from '~/types/app.types';
import { Privilege } from '~/types/auth.types';
import { BreadCrumbItem } from '~/types/navigation.types';
import { RequestDomains } from '~/types/station.types';
import { copyToClipboard } from '~/utils/app.utils';

const props = withDefaults(defineProps<PageProps>(), {
  title: undefined,
  breadcrumbs: () => [],
});
const router = useRouter();
const canister: Ref<ExternalCanister | null> = ref(null);
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

const dialogs = ref({ settings: false, unlink: false });
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
</script>
