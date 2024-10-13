<template>
  <PageLayout>
    <template #main-header>
      <PageHeader :title="pageTitle" :breadcrumbs="props.breadcrumbs">
        <template #actions>
          <AuthCheck :privileges="[Privilege.AddAsset]">
            <AssetDialogBtn :text="$t('pages.assets.btn_new_entry')" />
          </AuthCheck>
        </template>
      </PageHeader>
    </template>
    <template #main-body>
      <PageBody>
        <AuthCheck :privileges="[Privilege.ListRequests]">
          <RecentRequests
            class="mb-4"
            :see-all-link="{
              name: Routes.Requests,
              query: { group_by: RequestDomains.Assets },
            }"
            :types="[{ AddAsset: null }, { EditAsset: null }, { RemoveAsset: null }]"
            hide-not-found
          />
        </AuthCheck>

        <DataLoader
          v-slot="{ loading }"
          v-model:force-reload="forceReload"
          :disable-refresh="disableRefresh"
          :load="fetchList"
          :refresh-interval-ms="5000"
          @loaded="
            result => {
              assets = result.assets;
              privileges = result.privileges;
            }
          "
        >
          <VDataTable
            class="elevation-2 rounded"
            :loading="loading"
            :headers="headers"
            :items="assets"
            :items-per-page="-1"
            :hover="true"
          >
            <template #bottom>
              <!--this hides the footer as pagination is not required-->
            </template>
            <template #item.symbol="{ item: asset }">
              {{ asset.symbol }} ({{ asset.name }})
            </template>
            <template #item.blockchain="{ item: asset }">
              {{ $t(`blockchains.${asset.blockchain.toLowerCase()}.name`) }}
            </template>
            <template #item.standards="{ item: asset }">
              {{ asset.standards.join(', ') }}
            </template>

            <template #item.actions="{ item: asset }">
              <div class="d-flex justify-end">
                <ActionBtn
                  v-if="hasDeletePrivilege(asset.id)"
                  v-model="asset.id"
                  :icon="mdiTrashCanOutline"
                  :submit="id => station.service.removeAsset({ asset_id: id })"
                  @failed="useOnFailedOperation"
                  @submitted="useOnSuccessfulOperation"
                />
                <AssetDialogBtn
                  :icon="!hasEditPrivilege(asset.id) ? mdiEye : mdiPencil"
                  :asset-id="asset.id"
                  :readonly="!hasEditPrivilege(asset.id)"
                  variant="flat"
                  color="default"
                  size="small"
                  @opened="disableRefresh = $event"
                />
              </div>
            </template>
          </VDataTable>
        </DataLoader>
        <VPagination
          v-model="pagination.selectedPage"
          class="mt-2"
          :length="pagination.totalPages"
          rounded
          density="comfortable"
          @update:model-value="triggerSearch"
        />
      </PageBody>
    </template>
  </PageLayout>
</template>

<script lang="ts" setup>
import { mdiEye, mdiPencil, mdiTrashCanOutline } from '@mdi/js';
import { computed, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { VDataTable, VPagination } from 'vuetify/components';
import AuthCheck from '~/components/AuthCheck.vue';
import DataLoader from '~/components/DataLoader.vue';
import PageLayout from '~/components/PageLayout.vue';
import AssetDialogBtn from '~/components/assets/AssetDialogBtn.vue';
import ActionBtn from '~/components/buttons/ActionBtn.vue';
import PageBody from '~/components/layouts/PageBody.vue';
import PageHeader from '~/components/layouts/PageHeader.vue';
import RecentRequests from '~/components/requests/RecentRequests.vue';
import { useFetchList, usePagination } from '~/composables/lists.composable';
import {
  useOnFailedOperation,
  useOnSuccessfulOperation,
} from '~/composables/notifications.composable';
import { Routes } from '~/configs/routes.config';
import { Asset, AssetCallerPrivileges, UUID } from '~/generated/station/station.did';
import { useStationStore } from '~/stores/station.store';
import type { PageProps, TableHeader } from '~/types/app.types';
import { Privilege } from '~/types/auth.types';
import { RequestDomains } from '~/types/station.types';
import { throttle } from '~/utils/helper.utils';

const props = withDefaults(defineProps<PageProps>(), { title: undefined, breadcrumbs: () => [] });
const station = useStationStore();
const i18n = useI18n();
const pageTitle = computed(() => props.title || i18n.t('pages.assets.title'));
const assets = ref<Asset[]>([]);
const privileges = ref<AssetCallerPrivileges[]>([]);
const disableRefresh = ref(false);
const forceReload = ref(false);
const pagination = usePagination();
const triggerSearch = throttle(() => (forceReload.value = true), 500);
const headers = ref<TableHeader[]>([
  { title: i18n.t('terms.symbol'), key: 'symbol', sortable: false },
  { title: i18n.t('terms.blockchain'), key: 'blockchain', sortable: false },
  { title: i18n.t('terms.standards'), key: 'standards', sortable: false },
  { title: '', key: 'actions', sortable: false },
]);

const hasEditPrivilege = (id: UUID): boolean => {
  const privilege = privileges.value.find(p => p.id === id);
  return !!privilege?.can_edit;
};

const hasDeletePrivilege = (id: UUID): boolean => {
  const privilege = privileges.value.find(p => p.id === id);
  return !!privilege?.can_delete;
};

let useVerifiedCall = false;

const fetchList = useFetchList(
  (offset, limit) => {
    const results = station.service.listAssets(
      {
        offset,
        limit,
      },
      useVerifiedCall,
    );

    useVerifiedCall = true;

    return results;
  },
  {
    pagination,
    getTotal: res => Number(res.total),
  },
);
</script>
