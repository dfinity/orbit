<template>
  <PageLayout>
    <template #main-header>
      <PageHeader :title="pageTitle" :breadcrumbs="props.breadcrumbs">
        <template #actions>
          <AuthCheck :privileges="[Privilege.AddUser]">
            <OpenUserBtn :text="$t('pages.users.btn_new_user')" />
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
              query: { group_by: RequestDomains.Users },
            }"
            :types="[{ AddUser: null }, { EditUser: null }]"
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
              users = result.users;
            }
          "
        >
          <VDataTable
            class="elevation-2 rounded"
            :loading="loading"
            :headers="headers"
            :items="users"
            :items-per-page="-1"
            :hover="true"
          >
            <template #bottom>
              <!-- This removes the bottom pagination since we want to display all the results -->
            </template>
            <template #item.name="{ item: user }">
              {{ user.name }}
            </template>
            <template #item.user_groups="{ item: user }">
              {{ user.groups.map(ug => ug.name).join(', ') }}
            </template>
            <template #item.status="{ item: user }">
              <UserStatusChip :status="fromUserStatusVariantToEnum(user.status)" />
            </template>
            <template #item.principals="{ item: user }">
              <template v-if="user.identities.length > 0">
                <TextOverflow :text="user.identities[0].toText()" />
                <VChip v-if="user.identities.length > 1" size="x-small" class="ml-2">
                  +{{ user.identities.length - 1 }}
                </VChip>
              </template>
              <template v-else>-</template>
            </template>
            <template #item.actions="{ item: user }">
              <div class="text-right">
                <OpenUserBtn
                  :icon="!hasEditPrivilege(user.id) ? mdiEye : mdiPencil"
                  :user-id="user.id"
                  :readonly="!hasEditPrivilege(user.id)"
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
import { mdiEye, mdiPencil } from '@mdi/js';
import { computed, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { useDisplay } from 'vuetify';
import { VChip, VDataTable } from 'vuetify/components';
import AuthCheck from '~/components/AuthCheck.vue';
import DataLoader from '~/components/DataLoader.vue';
import PageLayout from '~/components/PageLayout.vue';
import TextOverflow from '~/components/TextOverflow.vue';
import UserStatusChip from '~/components/chips/UserStatusChip.vue';
import PageBody from '~/components/layouts/PageBody.vue';
import PageHeader from '~/components/layouts/PageHeader.vue';
import RecentRequests from '~/components/requests/RecentRequests.vue';
import OpenUserBtn from '~/components/users/OpenUserBtn.vue';
import { useFetchList, usePagination } from '~/composables/lists.composable';
import { Routes } from '~/configs/routes.config';
import { UUID, User, UserCallerPrivileges } from '~/generated/station/station.did';
import { fromUserStatusVariantToEnum } from '~/mappers/users.mapper';
import { useStationStore } from '~/stores/station.store';
import type { PageProps, TableHeader } from '~/types/app.types';
import { Privilege } from '~/types/auth.types';
import { RequestDomains } from '~/types/station.types';
import { throttle } from '~/utils/helper.utils';

const props = withDefaults(defineProps<PageProps>(), { title: undefined, breadcrumbs: () => [] });
const i18n = useI18n();
const pageTitle = computed(() => props.title || i18n.t('pages.users.title'));
const station = useStationStore();
const users = ref<User[]>([]);
const privileges = ref<UserCallerPrivileges[]>([]);
const forceReload = ref(false);
const disableRefresh = ref(false);
const pagination = usePagination();
const triggerSearch = throttle(() => (forceReload.value = true), 500);
const headerProps: { class: string } = { class: 'font-weight-bold' };

const { xs } = useDisplay();
const headers = computed((): TableHeader[] => [
  { title: i18n.t('terms.name'), key: 'name', headerProps },
  { title: i18n.t('terms.status'), key: 'status', headerProps },
  { title: i18n.t('terms.user_groups'), key: 'user_groups', headerProps },
  ...(xs.value ? [] : [{ title: i18n.t('terms.identity'), key: 'principals', headerProps }]),
  { title: '', key: 'actions', headerProps },
]);

const hasEditPrivilege = (id: UUID): boolean =>
  privileges.value.find(p => p.id === id)?.can_edit ?? false;

let useVerifiedCall = false;

const fetchList = useFetchList(
  (offset, limit) => {
    const results = station.service.listUsers(
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
