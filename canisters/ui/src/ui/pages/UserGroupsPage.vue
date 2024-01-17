<template>
  <PageLayout>
    <template #main-header>
      <VContainer class="pt-8 pb-8 pl-8 pr-8" fluid>
        <VRow>
          <VCol cols="12" md="6">
            <h1 class="text-h4">{{ $t('pages.user_groups.title') }}</h1>
          </VCol>
          <VCol cols="12" md="6" class="d-flex justify-end">
            <AuthCheck :privileges="[Privilege.AddUserGroup]">
              <ActionBtn
                v-model="createModel"
                :text="$t('pages.user_groups.btn_new_group')"
                :title="$t('pages.user_groups.create_new_group_title')"
                :submit="
                  ({ model }) =>
                    wallet.service.addUserGroup({
                      name: assertAndReturn(model.name).trim(),
                    })
                "
                color="primary-variant"
                size="default"
                variant="outlined"
                @failed="onFailedOperation"
                @submitted="onSuccessfulOperation"
              >
                <template #default="{ model: elem, submit }">
                  <UserGroupForm
                    v-model="elem.value.model"
                    @valid="isValid => (elem.value.valid = isValid)"
                    @submit="submit"
                  />
                </template>
                <template #actions="{ submit, loading: saving, model: elem }">
                  <VSpacer />
                  <VBtn
                    :loading="saving"
                    :disabled="!elem.value.valid"
                    color="primary"
                    variant="flat"
                    @click="submit"
                  >
                    {{ $t('terms.create') }}
                  </VBtn>
                </template>
              </ActionBtn>
            </AuthCheck>

            <AuthCheck :privileges="[Privilege.AddAccessPolicy]">
              <VBtn color="primary-variant" variant="flat" class="ml-2">
                {{ $t('pages.user_groups.btn_manage_permissions') }}
              </VBtn>
            </AuthCheck>
          </VCol>
          <VCol cols="12">
            <VDivider class="border-opacity-50" :thickness="2" />
          </VCol>
        </VRow>
      </VContainer>
    </template>
    <template #main-body>
      <VContainer class="px-8" fluid>
        <VRow>
          <VCol cols="12">
            <DataLoader
              :load="() => wallet.service.listUserGroups()"
              :error-msg="$t('pages.user_groups.error_loading_user_groups')"
              :refresh-interval-ms="5000"
            >
              <template #default="{ data, loading }">
                <VDataTable
                  hover
                  :headers="headers"
                  :loading="loading"
                  :items="data ? transformItems(data) : undefined"
                  :items-per-page="-1"
                >
                  <template #bottom>
                    <!-- This removes the bottom pagination since we want to display all the results -->
                  </template>
                  <template #item.actions="{ item }">
                    <div class="text-right">
                      <ActionBtn
                        v-model="item.id"
                        :icon="mdiTrashCanOutline"
                        :submit="id => wallet.service.removeUserGroup({ user_group_id: id })"
                        @failed="onFailedOperation"
                        @submitted="onSuccessfulOperation"
                      />

                      <ActionBtn
                        v-model="item.edit"
                        :title="$t('pages.user_groups.btn_edit_title')"
                        :icon="mdiPencil"
                        :submit="
                          changes =>
                            wallet.service.editUserGroup({
                              user_group_id: item.id,
                              name: changes.model.name.trim(),
                            })
                        "
                        @failed="onFailedOperation"
                        @submitted="onSuccessfulOperation"
                      >
                        <template #default="{ model: elem, submit }">
                          <UserGroupForm
                            v-model="elem.value.model"
                            @valid="isValid => (elem.value.valid = isValid)"
                            @submit="submit"
                          />
                        </template>
                        <template #actions="{ submit, loading: saving, model: elem }">
                          <VSpacer />
                          <VBtn
                            :loading="saving"
                            :disabled="!elem.value.valid"
                            color="primary"
                            variant="flat"
                            @click="submit"
                          >
                            {{ $t('terms.save') }}
                          </VBtn>
                        </template>
                      </ActionBtn>
                    </div>
                  </template>
                </VDataTable>
              </template>
            </DataLoader>
          </VCol>
        </VRow>
      </VContainer>
    </template>
  </PageLayout>
</template>

<script lang="ts" setup>
import { mdiPencil, mdiTrashCanOutline } from '@mdi/js';
import { ref } from 'vue';
import { Proposal, UserGroup } from '~/generated/wallet/wallet.did';
import { Privilege } from '~/types';
import AuthCheck from '~/ui/components/AuthCheck.vue';
import DataLoader from '~/ui/components/DataLoader.vue';
import PageLayout from '~/ui/components/PageLayout.vue';
import ActionBtn from '~/ui/components/buttons/ActionBtn.vue';
import UserGroupForm from '~/ui/components/forms/UserGroupForm.vue';
import { i18n } from '~/ui/modules/i18n';
import { useAppStore } from '~/ui/stores/app';
import { useWalletStore } from '~/ui/stores/wallet';
import { assertAndReturn } from '~/ui/utils';

const wallet = useWalletStore();
const app = useAppStore();

const headerProps: { class: string } = { class: 'font-weight-bold' };
const headers = ref<{ title: string; key: string; headerProps: { class: string } }[]>([
  { title: i18n.global.t('terms.user_group'), key: 'name', headerProps },
  { title: '', key: 'actions', headerProps },
]);

const transformItems = (items: UserGroup[]) => {
  return items.map(item => ({
    name: item.name,
    id: item.id,
    edit: {
      model: JSON.parse(JSON.stringify(item)) as UserGroup,
      valid: false,
    },
  }));
};

const onFailedOperation = (): void => {
  app.sendNotification({
    type: 'error',
    message: i18n.global.t('app.request_failed_message'),
  });
};

const onSuccessfulOperation = (proposal?: Proposal): void => {
  if (proposal && 'Rejected' in proposal.status) {
    app.sendNotification({
      type: 'error',
      message: i18n.global.t('app.request_rejected_message'),
    });

    return;
  }

  if (proposal && 'Adopted' in proposal.status) {
    app.sendNotification({
      type: 'success',
      message: i18n.global.t('app.request_adopted_message'),
    });

    return;
  }

  app.sendNotification({
    type: 'warning',
    message: i18n.global.t('app.request_pending_message'),
  });
};

const createModel = ref<{ valid: boolean; model: Partial<UserGroup> }>({
  model: {},
  valid: false,
});
</script>
