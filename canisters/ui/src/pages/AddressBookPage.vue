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
        <DataLoader
          v-slot="{ loading }"
          v-model:force-reload="forceReload"
          :disable-refresh="disableRefresh"
          :load="fetchList"
          :refresh-interval-ms="5000"
          @loaded="
            data => {
              addressBookEntries = data.address_book_entries;
              privileges = data.privileges;
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
            <template #item.blockchain="{ item }">
              {{ $t(`blockchains.${item.blockchain.toLowerCase()}.name`) }}
            </template>
            <template #item.actions="{ item }">
              <div class="d-flex justify-end">
                <ActionBtn
                  v-if="hasDeletePrivilege(item.id)"
                  v-model="item.id"
                  :icon="mdiTrashCanOutline"
                  :submit="id => wallet.service.removeAddressBookEntry(id)"
                  @failed="useOnFailedOperation"
                  @submitted="useOnSuccessfulOperation"
                />
                <AddressBookEntryBtn
                  :icon="!hasEditPrivilege(item.id) ? mdiEye : mdiPencil"
                  :address-book-entry-id="item.id"
                  :readonly="!hasEditPrivilege(item.id)"
                  variant="flat"
                  color="default"
                  size="small"
                  @opened="disableRefresh = $event"
                />
              </div>
            </template>
            <template #item.address="{ item }">
              <div class="d-flex align-center flex-no-wrap">
                <TextOverflow :max-length="app.isMobile ? 16 : 32" :text="item.address" />
                <VBtn
                  size="x-small"
                  variant="text"
                  :icon="mdiContentCopy"
                  @click="
                    copyToClipboard({
                      textToCopy: item.address,
                      sendNotification: true,
                    })
                  "
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
import { useFetchList, usePagination } from '~/composables/lists.composable';
import {
  useOnFailedOperation,
  useOnSuccessfulOperation,
} from '~/composables/notifications.composable';
import {
  AddressBookEntry,
  AddressBookEntryCallerPrivileges,
  UUID,
} from '~/generated/wallet/wallet.did';
import { useAppStore } from '~/stores/app.store';
import { useWalletStore } from '~/stores/wallet.store';
import { TableHeader } from '~/types/app.types';
import { Privilege } from '~/types/auth.types';
import { BreadCrumbItem } from '~/types/navigation.types';
import { copyToClipboard } from '~/utils/app.utils';
import { throttle } from '~/utils/helper.utils';

const props = withDefaults(
  defineProps<{
    title?: string;
    breadcrumbs?: BreadCrumbItem[];
  }>(),
  {
    title: undefined,
    breadcrumbs: () => [],
  },
);

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
const headers: TableHeader[] = [
  { title: i18n.t('terms.blockchain'), key: 'blockchain', sortable: false },
  { title: i18n.t('terms.address_owner'), key: 'address_owner', sortable: false },
  { title: i18n.t('terms.address'), key: 'address', sortable: false },
  { title: '', key: 'actions', sortable: false },
];

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
