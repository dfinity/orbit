<template>
  <PageLayout>
    <template #main-header>
      <PageHeader :title="title" :breadcrumbs="props.breadcrumbs" />
    </template>

    <template #main-body>
      <PageBody>
        <VContainer fluid class="px-0">
          <VRow>
            <VCol cols="12">
              <RecentRequests
                :see-all-link="{
                  name: Routes.Requests,
                  query: { group_by: RequestDomains.System },
                }"
                :types="[{ EditPermission: null }]"
                hide-not-found
              />
            </VCol>
            <VCol cols="12">
              <DataLoader
                v-slot="{ data, loading }"
                :load="() => fetchPermissions(useResourcesFromAggregatedView(resourcePermissions))"
                :refresh-interval-ms="5000"
                :disable-refresh="disableRefresh"
              >
                <PermissionList
                  :loading="loading"
                  :resources="resourcePermissions"
                  :permissions="data ? data.permissions : []"
                  :privileges="data ? data.privileges : []"
                  :preload-user-groups="data ? data.userGroups : []"
                  :preload-users="data ? data.users : []"
                  @editing="disableRefresh = $event"
                />
              </DataLoader>
            </VCol>
            <VCol cols="12">
              <div class="text-h6 px-2">{{ $t('permissions.individual_resources_title') }}</div>

              <VAutocomplete
                v-model="individualResourceSelected"
                class="px-2 mt-2"
                :items="individualResources"
                density="comfortable"
                :label="$t('permissions.select_resource')"
                hide-details
              />

              <IndividualAccountPermissions
                v-if="individualResourceSelected === ResourceTypeEnum.Account"
                :fetch-permissions="fetchPermissions"
              />
              <IndividualUserPermissions
                v-else-if="individualResourceSelected === ResourceTypeEnum.User"
                :fetch-permissions="fetchPermissions"
              />
              <IndividualUserGroupPermissions
                v-else-if="individualResourceSelected === ResourceTypeEnum.UserGroup"
                :fetch-permissions="fetchPermissions"
              />
            </VCol>
          </VRow>
        </VContainer>
      </PageBody>
    </template>
  </PageLayout>
</template>

<script lang="ts" setup>
import { computed, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { VAutocomplete, VCol, VContainer, VRow } from 'vuetify/components';
import DataLoader from '~/components/DataLoader.vue';
import PageBody from '~/components/layouts/PageBody.vue';
import PageHeader from '~/components/layouts/PageHeader.vue';
import PageLayout from '~/components/PageLayout.vue';
import IndividualAccountPermissions from '~/components/permissions/IndividualAccountPermissions.vue';
import IndividualUserGroupPermissions from '~/components/permissions/IndividualUserGroupPermissions.vue';
import IndividualUserPermissions from '~/components/permissions/IndividualUserPermissions.vue';
import PermissionList from '~/components/permissions/PermissionList.vue';
import RecentRequests from '~/components/requests/RecentRequests.vue';
import { useResourcesFromAggregatedView } from '~/composables/permissions.composable';
import { globalPermissions } from '~/configs/permissions.config';
import { Routes } from '~/configs/routes.config';
import {
  BasicUser,
  Permission,
  PermissionCallerPrivileges,
  Resource,
  UserGroup,
} from '~/generated/station/station.did';
import { useStationStore } from '~/stores/station.store';
import type { PageProps } from '~/types/app.types';
import { ResourceTypeEnum } from '~/types/permissions.types';
import { RequestDomains } from '~/types/station.types';

const props = withDefaults(defineProps<PageProps>(), {
  title: undefined,
  breadcrumbs: () => [],
});

const i18n = useI18n();
const title = computed(() => props.title || i18n.t('pages.permissions.title'));
const station = useStationStore();
const disableRefresh = ref(false);
const resourcePermissions = globalPermissions();
const individualResourceSelected = ref<ResourceTypeEnum | null>(null);
const individualResourceKeys = ref<ResourceTypeEnum[]>([
  ResourceTypeEnum.Account,
  ResourceTypeEnum.User,
  ResourceTypeEnum.UserGroup,
]);

const individualResources = computed(() => {
  return individualResourceKeys.value.map(key => ({
    value: key,
    title: i18n.t(`permissions.resources.${key.toLowerCase()}`),
  }));
});

const fetchPermissions = async (
  resources: Resource[],
): Promise<{
  permissions: Permission[];
  privileges: PermissionCallerPrivileges[];
  userGroups: UserGroup[];
  users: BasicUser[];
}> => {
  const userGroups: UserGroup[] = [];
  const users: BasicUser[] = [];
  let permissions: Permission[] = [];
  let privileges: PermissionCallerPrivileges[] = [];
  let limit = 250;
  let nextOffset = BigInt(0);
  let maxOffsetFound = nextOffset;

  do {
    // This is to avoid infinite loops in case the offset is not updated properly
    maxOffsetFound = nextOffset;

    const result = await station.service.listPermissions({
      resources: [resources],
      paginate: [
        {
          limit: [limit],
          offset: [nextOffset],
        },
      ],
    });

    userGroups.push(...result.user_groups);
    users.push(...result.users);
    privileges.push(...result.privileges);

    permissions = permissions.concat(result.permissions);
    nextOffset =
      result.next_offset?.[0] !== undefined && result.next_offset[0] > 0
        ? result.next_offset[0]
        : BigInt(-1);
  } while (nextOffset > 0 && nextOffset > maxOffsetFound);

  return { permissions, userGroups, users, privileges };
};
</script>
