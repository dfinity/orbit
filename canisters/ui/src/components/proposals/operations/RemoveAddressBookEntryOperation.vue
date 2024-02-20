<template>
  <div v-if="isListMode" class="d-flex flex-column ga-0 text-caption">
    <ProposalOperationListRow v-if="formValue.id">
      <template #name>{{ $t('terms.id') }}</template>
      <template #content>
        {{ formValue.id }}
      </template>
    </ProposalOperationListRow>
  </div>
  <VProgressCircular v-else-if="loading" />
  <AddressBookForm v-else :model-value="formValue" mode="view" />
</template>

<script setup lang="ts">
import { Ref, computed, onBeforeMount, ref } from 'vue';
import AddressBookForm from '~/components/address-book/AddressBookForm.vue';
import logger from '~/core/logger.core';
import {
  AddressBookEntry,
  Proposal,
  RemoveAddressBookEntryOperation,
} from '~/generated/wallet/wallet.did';
import { useWalletStore } from '~/stores/wallet.store';
import ProposalOperationListRow from '../ProposalOperationListRow.vue';

const props = withDefaults(
  defineProps<{
    proposal: Proposal;
    operation: RemoveAddressBookEntryOperation;
    mode?: 'list' | 'detail';
  }>(),
  {
    mode: 'list',
  },
);

const isListMode = computed(() => props.mode === 'list');
const formValue: Ref<Partial<AddressBookEntry>> = ref({});
const loading = ref(false);
const wallet = useWalletStore();

const fetchDetails = async () => {
  try {
    if (loading.value || isListMode.value) {
      return;
    }

    loading.value = true;
    const currentEntry = await wallet.service.getAddressBookEntry({
      address_book_entry_id: props.operation.input.address_book_entry_id,
    });

    formValue.value = currentEntry.address_book_entry;
  } catch (e) {
    logger.error('Failed to fetch address book entry details', e);
  } finally {
    loading.value = false;
  }
};

onBeforeMount(() => {
  const entry: Partial<AddressBookEntry> = {};
  entry.id = props.operation.input.address_book_entry_id;

  formValue.value = entry;

  fetchDetails();
});
</script>
