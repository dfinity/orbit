<template>
  <PageLayout>
    <template #main-header>
      <PageHeader :title="$t('pages.users.title')">
        <template #actions>
          <AuthCheck :privileges="[Privilege.AddUser]">
            <ActionBtn
              v-model="createModel"
              :text="$t('pages.users.btn_new_user')"
              :title="$t('pages.users.create_new_user_title')"
              :submit="
                ({ model }) =>
                  wallet.service.addUser({
                    name: model.name ? [model.name.trim()] : [],
                    status: assertAndReturn(model.status),
                    groups: model.groups ?? [],
                    identities: model.identities
                      ? model.identities.map(id => Principal.fromText(id))
                      : [],
                  })
              "
              color="primary-variant"
              size="default"
              variant="outlined"
              data-test-id="create-user-btn"
              @failed="onFailedOperation"
              @submitted="onSuccessfulOperation"
            >
              <template #default="{ model: elem, submit }">
                <UserForm
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
        </template>
      </PageHeader>
    </template>
    <template #main-body>
      <PageBody>
        <DataLoader
          :load="fetchData"
          :error-msg="$t('pages.users.error_fetching_users')"
          :refresh-interval-ms="10000"
        >
          <template #default="{ data, loading }">
            <VDataTable
              hover
              :headers="headers"
              :loading="loading"
              :items="data ? transformItems(data.users) : undefined"
              :items-per-page="-1"
            >
              <template #bottom>
                <!-- This removes the bottom pagination since we want to display all the results -->
              </template>
              <template #item.status="{ item }">
                <UserStatusChip :status="item.status" />
              </template>
              <template #item.principals="{ item }">
                <template v-if="item.principals.length > 0">
                  <span>{{ item.principals[0] }}</span>
                  <VChip v-if="item.principals.length > 1" size="x-small" class="ml-2">
                    +{{ item.principals.length - 1 }}
                  </VChip>
                </template>
                <template v-else>-</template>
              </template>
              <template #item.actions="{ item }">
                <div class="text-right">
                  <ActionBtn
                    v-model="item.edit"
                    :title="$t('pages.users.btn_edit_title')"
                    :icon="mdiPencil"
                    :submit="
                      changes =>
                        wallet.service.editUser({
                          id: item.id,
                          name: changes.model.name ? [changes.model.name.trim()] : [],
                          groups: changes.model.groups ? [changes.model.groups] : [],
                          identities: changes.model.identities
                            ? [changes.model.identities.map(id => Principal.fromText(id))]
                            : [],
                        })
                    "
                    data-test-id="edit-user-btn"
                    @failed="onFailedOperation"
                    @submitted="onSuccessfulOperation"
                  >
                    <template #default="{ model: elem, submit }">
                      <UserForm
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
      </PageBody>
    </template>
  </PageLayout>
</template>

<script lang="ts" setup>
import { Principal } from '@dfinity/principal';
import { mdiPencil } from '@mdi/js';
import { ref } from 'vue';
import { Proposal, User } from '~/generated/wallet/wallet.did';
import { fromUserStatusVariantToEnum, fromUserToUserInput } from '~/mappers/users.mapper';
import { Privilege, UserInput } from '~/types';
import AuthCheck from '~/ui/components/AuthCheck.vue';
import DataLoader from '~/ui/components/DataLoader.vue';
import PageLayout from '~/ui/components/PageLayout.vue';
import ActionBtn from '~/ui/components/buttons/ActionBtn.vue';
import UserStatusChip from '~/ui/components/chips/UserStatusChip.vue';
import UserForm from '~/ui/components/forms/UserForm.vue';
import PageBody from '~/ui/components/layouts/PageBody.vue';
import PageHeader from '~/ui/components/layouts/PageHeader.vue';
import { i18n } from '~/ui/modules/i18n';
import { useAppStore } from '~/ui/stores/app';
import { useWalletStore } from '~/ui/stores/wallet';
import { assertAndReturn } from '~/ui/utils';

const wallet = useWalletStore();
const app = useAppStore();

const headerProps: { class: string } = { class: 'font-weight-bold' };
const headers = ref<{ title: string; key: string; headerProps: { class: string } }[]>([
  { title: i18n.global.t('terms.name'), key: 'name', headerProps },
  { title: i18n.global.t('terms.status'), key: 'status', headerProps },
  { title: i18n.global.t('terms.principal'), key: 'principals', headerProps },
  { title: '', key: 'actions', headerProps },
]);

const fetchData = async (): Promise<{ users: User[] }> => {
  let limit = 100;
  let nextOffset = 0;
  let users: User[] = [];
  let maxOffsetFound = nextOffset;

  do {
    // This is to avoid infinite loops in case the offset is not updated properly
    maxOffsetFound = nextOffset;

    const { users: usersChunk, next_offset } = await wallet.service.listUsers({
      limit,
      offset: nextOffset,
    });
    users.push(...usersChunk);

    nextOffset = next_offset?.[0] !== undefined && next_offset[0] > 0 ? Number(next_offset[0]) : -1;
  } while (nextOffset > 0 && nextOffset > maxOffsetFound);

  return { users };
};

const transformItems = (items: User[]) => {
  return items.map(item => ({
    name: item.name?.[0] ?? '-',
    id: item.id,
    status: fromUserStatusVariantToEnum(item.status),
    principals: item.identities?.map(id => id.toText()) ?? [],
    edit: {
      model: fromUserToUserInput(item),
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

const createModel = ref<{
  valid: boolean;
  model: Partial<UserInput>;
}>({
  model: fromUserToUserInput({}),
  valid: false,
});
</script>
