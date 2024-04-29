<template>
  <PageLayout>
    <template #main-header>
      <PageHeader :title="pageTitle" :breadcrumbs="props.breadcrumbs">
        <template #actions>
          <AuthCheck :privileges="[Privilege.AddUserGroup]">
            <OpenUserGroupBtn :text="$t('pages.user_groups.btn_new_group')" variant="outlined" />
          </AuthCheck>

          <AuthCheck :privileges="[Privilege.ListPermissions]">
            <VBtn
              color="primary"
              data-test-id="manage-permissions-btn"
              :to="{ name: Routes.Permissions }"
            >
              {{ $t('pages.user_groups.btn_manage_permissions') }}
            </VBtn>
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
              query: { group_by: RequestDomains.System },
            }"
            :types="[{ AddUserGroup: null }, { EditUserGroup: null }, { RemoveUserGroup: null }]"
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
              privileges = result.privileges;
              userGroups = result.user_groups;
            }
          "
        >
          <VDataTable
            class="elevation-2 rounded"
            :loading="loading"
            :headers="headers"
            :items="userGroups"
            :items-per-page="-1"
            :hover="true"
          >
            <template #bottom>
              <!-- This removes the bottom pagination since we want to display all the results -->
            </template>
            <template #item.actions="{ item: userGroup }">
              <div class="text-right">
                <ActionBtn
                  v-if="hasDeletePrivilege(userGroup.id)"
                  v-model="userGroup.id"
                  :icon="mdiTrashCanOutline"
                  :submit="id => station.service.removeUserGroup({ user_group_id: id })"
                  @failed="useOnFailedOperation"
                  @submitted="useOnSuccessfulOperation"
                />
                <OpenUserGroupBtn
                  :icon="!hasEditPrivilege(userGroup.id) ? mdiEye : mdiPencil"
                  :user-group-id="userGroup.id"
                  :readonly="!hasEditPrivilege(userGroup.id)"
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
import { VBtn, VDataTable, VPagination } from 'vuetify/components';
import AuthCheck from '~/components/AuthCheck.vue';
import DataLoader from '~/components/DataLoader.vue';
import PageLayout from '~/components/PageLayout.vue';
import ActionBtn from '~/components/buttons/ActionBtn.vue';
import PageBody from '~/components/layouts/PageBody.vue';
import PageHeader from '~/components/layouts/PageHeader.vue';
import RecentRequests from '~/components/requests/RecentRequests.vue';
import OpenUserGroupBtn from '~/components/users/OpenUserGroupBtn.vue';
import { useFetchList, usePagination } from '~/composables/lists.composable';
import {
  useOnFailedOperation,
  useOnSuccessfulOperation,
} from '~/composables/notifications.composable';
import { Routes } from '~/configs/routes.config';
import { UUID, UserGroup, UserGroupCallerPrivileges } from '~/generated/station/station.did';
import { useStationStore } from '~/stores/station.store';
import type { PageProps, TableHeader } from '~/types/app.types';
import { Privilege } from '~/types/auth.types';
import { RequestDomains } from '~/types/station.types';
import { throttle } from '~/utils/helper.utils';

const props = withDefaults(defineProps<PageProps>(), { title: undefined, breadcrumbs: () => [] });
const i18n = useI18n();
const pageTitle = computed(() => props.title || i18n.t('pages.user_groups.title'));
const station = useStationStore();
const userGroups = ref<UserGroup[]>([]);
const privileges = ref<UserGroupCallerPrivileges[]>([]);
const forceReload = ref(false);
const disableRefresh = ref(false);
const pagination = usePagination();
const triggerSearch = throttle(() => (forceReload.value = true), 500);
const headerProps: { class: string } = { class: 'font-weight-bold' };
const headers = ref<TableHeader[]>([
  { title: i18n.t('terms.user_group'), key: 'name', headerProps },
  { title: '', key: 'actions', headerProps },
]);

const hasEditPrivilege = (id: UUID): boolean =>
  privileges.value.find(p => p.id === id)?.can_edit ?? false;

const hasDeletePrivilege = (id: UUID): boolean =>
  privileges.value.find(p => p.id === id)?.can_delete ?? false;

let useVerifiedCall = false;

const fetchList = useFetchList(
  (offset, limit) => {
    const results = station.service.listUserGroups(
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
