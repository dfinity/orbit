<template>
  <PageLayout>
    <template #main-header>
      <PageHeader :title="pageTitle" :breadcrumbs="props.breadcrumbs">
        <template #actions>
          <AuthCheck :privileges="[Privilege.AddAddressBookEntry]">
            <AddressBookEntryBtn :text="$t('terms.new_address')" variant="outlined" />
          </AuthCheck>
        </template>
      </PageHeader>
    </template>
    <template #main-body>
      <PageBody>
        <AuthCheck :privileges="[Privilege.ListProposals]">
          <RecentProposals
            class="mb-4"
            :see-all-link="{
              name: Routes.Proposals,
              query: { group_by: ProposalDomains.AddressBook },
            }"
            :types="[
              { AddAddressBookEntry: null },
              { EditAddressBookEntry: null },
              { RemoveAddressBookEntry: null },
            ]"
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
              addressBookEntries = result.address_book_entries;
              privileges = result.privileges;
            }
          "
        >
          <VDataTable
            :loading="loading"
            :headers="headers"
            :items="addressBookEntries"
            :items-per-page="-1"
            :hover="true"
          >
            <template #bottom>
              <!--this hides the footer as pagination is not required-->
            </template>
            <template #item.blockchain="{ item: addressBookEntry }">
              {{ $t(`blockchains.${addressBookEntry.blockchain.toLowerCase()}.name`) }}
            </template>
            <template #item.address="{ item: addressBookEntry }">
              <div class="d-flex align-center flex-no-wrap">
                <TextOverflow
                  :max-length="app.isMobile ? 16 : 32"
                  :text="addressBookEntry.address"
                />
                <VBtn
                  size="x-small"
                  variant="text"
                  :icon="mdiContentCopy"
                  @click="
                    copyToClipboard({
                      textToCopy: addressBookEntry.address,
                      sendNotification: true,
                    })
                  "
                />
              </div>
            </template>
            <template #item.actions="{ item: addressBookEntry }">
              <div class="d-flex justify-end">
                <ActionBtn
                  v-if="hasDeletePrivilege(addressBookEntry.id)"
                  v-model="addressBookEntry.id"
                  :icon="mdiTrashCanOutline"
                  :submit="id => wallet.service.removeAddressBookEntry(id)"
                  @failed="useOnFailedOperation"
                  @submitted="useOnSuccessfulOperation"
                />
                <AddressBookEntryBtn
                  :icon="!hasEditPrivilege(addressBookEntry.id) ? mdiEye : mdiPencil"
                  :address-book-entry-id="addressBookEntry.id"
                  :readonly="!hasEditPrivilege(addressBookEntry.id)"
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
import { mdiContentCopy, mdiEye, mdiPencil, mdiTrashCanOutline } from '@mdi/js';
import { computed, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import AuthCheck from '~/components/AuthCheck.vue';
import DataLoader from '~/components/DataLoader.vue';
import PageLayout from '~/components/PageLayout.vue';
import TextOverflow from '~/components/TextOverflow.vue';
import AddressBookEntryBtn from '~/components/address-book/AddressBookEntryBtn.vue';
import ActionBtn from '~/components/buttons/ActionBtn.vue';
import PageBody from '~/components/layouts/PageBody.vue';
import PageHeader from '~/components/layouts/PageHeader.vue';
import RecentProposals from '~/components/proposals/RecentProposals.vue';
import { useFetchList, usePagination } from '~/composables/lists.composable';
import {
  useOnFailedOperation,
  useOnSuccessfulOperation,
} from '~/composables/notifications.composable';
import { Routes } from '~/configs/routes.config';
import {
  AddressBookEntry,
  AddressBookEntryCallerPrivileges,
  UUID,
} from '~/generated/wallet/wallet.did';
import { useAppStore } from '~/stores/app.store';
import { useWalletStore } from '~/stores/wallet.store';
import type { PageProps, TableHeader } from '~/types/app.types';
import { Privilege } from '~/types/auth.types';
import { ProposalDomains } from '~/types/wallet.types';
import { copyToClipboard } from '~/utils/app.utils';
import { throttle } from '~/utils/helper.utils';

const props = withDefaults(defineProps<PageProps>(), { title: undefined, breadcrumbs: () => [] });
const app = useAppStore();
const wallet = useWalletStore();
const i18n = useI18n();
const pageTitle = computed(() => props.title || i18n.t('pages.address_book.title'));
const addressBookEntries = ref<AddressBookEntry[]>([]);
const privileges = ref<AddressBookEntryCallerPrivileges[]>([]);
const disableRefresh = ref(false);
const forceReload = ref(false);
const pagination = usePagination();
const triggerSearch = throttle(() => (forceReload.value = true), 500);
const headers = ref<TableHeader[]>([
  { title: i18n.t('terms.blockchain'), key: 'blockchain', sortable: false },
  { title: i18n.t('terms.address_owner'), key: 'address_owner', sortable: false },
  { title: i18n.t('terms.address'), key: 'address', sortable: false },
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

const fetchList = useFetchList(
  (offset, limit) => {
    return wallet.service.listAddressBook({
      offset,
      limit,
    });
  },
  {
    pagination,
    getTotal: res => Number(res.total),
  },
);
</script>
