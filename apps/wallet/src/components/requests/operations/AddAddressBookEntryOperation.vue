<template>
  <div v-if="isListMode" class="d-flex flex-column ga-0 text-caption">
    <RequestOperationListRow>
      <template #name>{{ $t('terms.name') }}</template>
      <template #content>
        {{ formValue.address_owner ?? '-' }}
      </template>
    </RequestOperationListRow>
    <RequestOperationListRow v-if="formValue.address">
      <template #name>{{ $t('terms.address') }}</template>
      <template #content>
        <span class="text-break">
          {{ formValue.address }}
        </span>
      </template>
    </RequestOperationListRow>
  </div>
  <AddressBookForm v-else :model-value="formValue" mode="view" />
</template>

<script setup lang="ts">
import { Ref, computed, onBeforeMount, ref } from 'vue';
import {
  AddressBookEntry,
  AddAddressBookEntryOperation,
  Request,
} from '~/generated/station/station.did';
import RequestOperationListRow from '../RequestOperationListRow.vue';
import AddressBookForm from '~/components/address-book/AddressBookForm.vue';

const props = withDefaults(
  defineProps<{
    request: Request;
    operation: AddAddressBookEntryOperation;
    mode?: 'list' | 'detail';
  }>(),
  {
    mode: 'list',
  },
);

const isListMode = computed(() => props.mode === 'list');
const formValue: Ref<Partial<AddressBookEntry>> = ref({});

onBeforeMount(() => {
  const entry: Partial<AddressBookEntry> = {};
  entry.blockchain = props.operation.input.blockchain;
  entry.address_owner = props.operation.input.address_owner;
  entry.labels = props.operation.input.labels;
  entry.address = props.operation.input.address;
  entry.metadata = props.operation.input.metadata;

  formValue.value = entry;
});
</script>
