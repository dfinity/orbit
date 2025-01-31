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
            <VCol>
              <VRow>
                <VCol cols="12" class="px-6">
                  <h1 class="text-h4">{{ $t('permissions.global_permissions') }}</h1>
                  <VDivider class="pb-2" />
                  <p>{{ $t('permissions.global_permissions_description') }}</p>
                </VCol>
                <VCol cols="12" class="px-6">
                  <FlyoutDialog
                    v-model="dialog.open"
                    :closable="dialog.closable"
                    :title="$t('terms.permission')"
                  >
                    <div v-if="dialog.content" class="d-flex flex-column h-100">
                      <PermissionItemForm
                        v-model="dialog.content.allow"
                        :readonly="!dialog.content.editable"
                        :resource="dialog.content.resource"
                        class="flex-grow-1 py-4 px-6"
                        @submitting="dialog.closable = !$event"
                        @submitted="dialog.open = false"
                      >
                        <template #actions="{ valid, submitting, submit, edited }">
                          <VDivider />
                          <VCardActions class="pa-2">
                            <VBtn
                              color="primary"
                              variant="elevated"
                              data-test-id="submit-btn"
                              block
                              :disabled="!valid || !edited"
                              :loading="submitting"
                              @click.stop="submit"
                            >
                              {{ $t('terms.update') }}
                            </VBtn>
                          </VCardActions>
                        </template>
                      </PermissionItemForm>
                    </div>
                  </FlyoutDialog>
                  <DataLoader
                    v-slot="{ loading }"
                    :load="loadPermissions"
                    :refresh-interval-ms="5000"
                  >
                    <VProgressLinear v-if="loading" indeterminate />
                    <VExpansionPanels :disabled="loading">
                      <VExpansionPanel>
                        <VExpansionPanelTitle class="text-wrap">
                          <VIcon :icon="mdiWalletBifold" class="mr-2" size="small" />
                          <p>{{ $t('permissions.categories.treasury') }}</p>
                        </VExpansionPanelTitle>
                        <VExpansionPanelText>
                          <div class="d-flex flex-column ga-4">
                            <PermissionItem
                              v-for="(resource, idx) in Object.values(GLOBAL_PERMISSIONS.treasury)"
                              :key="idx"
                              :resource="resource"
                              :allowed="allowedByResource[JSON.stringify(resource)]"
                              :db="{ usersById, userGroupsById }"
                              class="cursor-pointer"
                              @click.stop="openDialog(resource)"
                            />
                          </div>
                        </VExpansionPanelText>
                      </VExpansionPanel>
                      <VExpansionPanel>
                        <VExpansionPanelTitle class="text-wrap">
                          <VIcon :icon="mdiDatabase" class="mr-2" size="small" />
                          <p>{{ $t('permissions.categories.canisters') }}</p>
                        </VExpansionPanelTitle>
                        <VExpansionPanelText>
                          <div class="d-flex flex-column ga-4">
                            <PermissionItem
                              v-for="(resource, idx) in Object.values(GLOBAL_PERMISSIONS.canisters)"
                              :key="idx"
                              :resource="resource"
                              :allowed="allowedByResource[JSON.stringify(resource)]"
                              :db="{ usersById, userGroupsById }"
                              class="cursor-pointer"
                              @click.stop="openDialog(resource)"
                            />
                          </div>
                        </VExpansionPanelText>
                      </VExpansionPanel>
                      <VExpansionPanel>
                        <VExpansionPanelTitle class="text-wrap">
                          <VIcon :icon="mdiAccountGroup" class="mr-2" size="small" />
                          <p>{{ $t('permissions.categories.users') }}</p>
                        </VExpansionPanelTitle>
                        <VExpansionPanelText>
                          <div class="d-flex flex-column ga-4">
                            <PermissionItem
                              v-for="(resource, idx) in Object.values(GLOBAL_PERMISSIONS.users)"
                              :key="idx"
                              :resource="resource"
                              :allowed="allowedByResource[JSON.stringify(resource)]"
                              :db="{ usersById, userGroupsById }"
                              class="cursor-pointer"
                              @click.stop="openDialog(resource)"
                            />
                          </div>
                        </VExpansionPanelText>
                      </VExpansionPanel>
                      <VExpansionPanel>
                        <VExpansionPanelTitle class="text-wrap">
                          <VIcon :icon="mdiCogs" class="mr-2" size="small" />
                          <p>{{ $t('permissions.categories.system') }}</p>
                        </VExpansionPanelTitle>
                        <VExpansionPanelText>
                          <div class="d-flex flex-column ga-4">
                            <PermissionItem
                              v-for="(resource, idx) in Object.values(GLOBAL_PERMISSIONS.system)"
                              :key="idx"
                              :resource="resource"
                              :allowed="allowedByResource[JSON.stringify(resource)]"
                              :db="{ usersById, userGroupsById }"
                              class="cursor-pointer"
                              @click.stop="openDialog(resource)"
                            />
                          </div>
                        </VExpansionPanelText>
                      </VExpansionPanel>
                    </VExpansionPanels>
                  </DataLoader>
                </VCol>
                <VCol cols="12" class="px-6">
                  <p class="d-flex flex-row ga-1 align-center text-body-2">
                    <VIcon :icon="mdiProgressPencil" size="small" class="text-medium-emphasis" />
                    <span>{{ $t('permissions.action_approval_legend') }}</span>
                  </p>
                </VCol>
              </VRow>
            </VCol>
          </VRow>
        </VContainer>
      </PageBody>
    </template>
  </PageLayout>
</template>

<script lang="ts" setup>
import { mdiAccountGroup, mdiCogs, mdiDatabase, mdiProgressPencil, mdiWalletBifold } from '@mdi/js';
import { computed, Ref, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { VCol, VContainer, VDivider, VRow } from 'vuetify/components';
import DataLoader from '~/components/DataLoader.vue';
import PageBody from '~/components/layouts/PageBody.vue';
import PageHeader from '~/components/layouts/PageHeader.vue';
import PageLayout from '~/components/PageLayout.vue';
import PermissionItem from '~/components/permissions/PermissionItem.vue';
import PermissionItemForm from '~/components/permissions/PermissionItemForm.vue';
import RecentRequests from '~/components/requests/RecentRequests.vue';
import FlyoutDialog from '~/components/ui/FlyoutDialog.vue';
import { GLOBAL_PERMISSIONS } from '~/configs/permissions.config';
import { Routes } from '~/configs/routes.config';
import { Allow, BasicUser, Resource, UserGroup } from '~/generated/station/station.did';
import { useStationStore } from '~/stores/station.store';
import type { PageProps } from '~/types/app.types';
import { RequestDomains } from '~/types/station.types';

const props = withDefaults(defineProps<PageProps>(), {
  title: undefined,
  breadcrumbs: () => [],
});

const i18n = useI18n();
const title = computed(() => props.title || i18n.t('pages.permissions.title'));
const resources = [
  ...Object.values(GLOBAL_PERMISSIONS.treasury),
  ...Object.values(GLOBAL_PERMISSIONS.canisters),
  ...Object.values(GLOBAL_PERMISSIONS.users),
  ...Object.values(GLOBAL_PERMISSIONS.system),
];

const station = useStationStore();
const allowedByResource: Ref<Record<string, Allow>> = ref({});
const privilegesByResource: Ref<Record<string, { canEdit: boolean }>> = ref({});
const usersById: Ref<Record<string, BasicUser>> = ref({});
const userGroupsById: Ref<Record<string, UserGroup>> = ref({});

const dialog: Ref<{
  open: boolean;
  closable: boolean;
  content?: {
    editable: boolean;
    resource: Resource;
    allow: Allow;
  };
}> = ref({ open: false, closable: true });

const openDialog = (resource: Resource): void => {
  dialog.value = {
    open: true,
    closable: true,
    content: {
      editable: privilegesByResource.value[JSON.stringify(resource)]?.canEdit ?? false,
      resource: resource,
      allow: allowedByResource.value[JSON.stringify(resource)] ?? {
        auth_scope: { Restricted: null },
        users: [],
        user_groups: [],
      },
    },
  };
};

// The very first call is a query call to get the initial details faster,
// then the subsequent calls are verified calls
const initialPageLoad = ref(false);

const loadPermissions = async (): Promise<boolean> => {
  const result = await station.service.listPermissions(
    {
      resources: [resources],
      paginate: [],
    },
    initialPageLoad.value,
  );

  allowedByResource.value = Object.fromEntries(
    result.permissions.map(permission => [JSON.stringify(permission.resource), permission.allow]),
  );
  privilegesByResource.value = Object.fromEntries(
    result.privileges.map(privilege => [
      JSON.stringify(privilege.resource),
      { canEdit: privilege.can_edit },
    ]),
  );
  usersById.value = Object.fromEntries(result.users.map(user => [user.id, user]));
  userGroupsById.value = Object.fromEntries(
    result.user_groups.map(userGroup => [userGroup.id, userGroup]),
  );

  if (initialPageLoad.value) {
    // For the initial page load we fetch the permissions with a query call and immediately after
    // we fetch the permissions with a verified call to update the UI with the certified data
    initialPageLoad.value = false;
    loadPermissions();
  }

  return true;
};
</script>
