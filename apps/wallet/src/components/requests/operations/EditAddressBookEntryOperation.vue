<template>
  <div v-if="isListMode" class="d-flex flex-column ga-0 text-caption">
    <RequestOperationListRow v-if="formValue.id">
      <template #name>{{ $t('terms.id') }}</template>
      <template #content>
        {{ formValue.id }}
      </template>
    </RequestOperationListRow>
    <RequestOperationListRow>
      <template #name>{{ $t('terms.name') }}</template>
      <template #content>
        {{ formValue.address_owner ?? '-' }}
      </template>
    </RequestOperationListRow>
    <RequestOperationListRow v-if="formValue.address">
      <template #name>{{ $t('terms.address') }}</template>
      <template #content>
        {{ formValue.address }}
      </template>
    </RequestOperationListRow>
  </div>
  <VProgressCircular indeterminate v-else-if="loading" />
  <AddressBookForm v-else :model-value="formValue" mode="view" />
</template>

<script setup lang="ts">
import { Ref, computed, onBeforeMount, ref } from 'vue';
import AddressBookForm from '~/components/address-book/AddressBookForm.vue';
import logger from '~/core/logger.core';
import {
  AddressBookEntry,
  EditAddressBookEntryOperation,
  Request,
} from '~/generated/station/station.did';
import { useStationStore } from '~/stores/station.store';
import { variantIs } from '~/utils/helper.utils';
import RequestOperationListRow from '../RequestOperationListRow.vue';
import { VProgressCircular } from 'vuetify/components';

const props = withDefaults(
  defineProps<{
    request: Request;
    operation: EditAddressBookEntryOperation;
    mode?: 'list' | 'detail';
  }>(),
  {
    mode: 'list',
  },
);

const isListMode = computed(() => props.mode === 'list');
const formValue: Ref<Partial<AddressBookEntry>> = ref({});
const loading = ref(false);
const station = useStationStore();

const fetchDetails = async () => {
  try {
    loading.value = true;
    const currentEntry = await station.service.getAddressBookEntry(
      {
        address_book_entry_id: props.operation.input.address_book_entry_id,
      },
      true,
    );

    let currentMetadata = currentEntry.address_book_entry.metadata;
    if (props.operation.input.change_metadata?.[0]) {
      const changeMetadata = props.operation.input.change_metadata[0];
      if (variantIs(changeMetadata, 'ReplaceAllBy')) {
        currentMetadata = changeMetadata.ReplaceAllBy;
      } else if (variantIs(changeMetadata, 'OverrideSpecifiedBy')) {
        changeMetadata.OverrideSpecifiedBy.forEach(metadata => {
          const existingValue = currentMetadata.find(m => m.key === metadata.key);
          if (existingValue) {
            existingValue.value = metadata.value;
          }
        });
      } else if (variantIs(changeMetadata, 'RemoveKeys')) {
        changeMetadata.RemoveKeys.forEach(metadata => {
          const existingValueIndex = currentMetadata.findIndex(m => m.key === metadata);
          if (existingValueIndex !== -1) {
            currentMetadata.splice(existingValueIndex, 1);
          }
        });
      }
    }

    formValue.value.metadata = currentMetadata;
  } catch (e) {
    logger.error('Failed to fetch address book entry details', e);
  } finally {
    loading.value = false;
  }
};

onBeforeMount(() => {
  const entry: Partial<AddressBookEntry> = {};
  entry.id = props.operation.input.address_book_entry_id;
  if (props.operation.input.address_owner?.[0]) {
    entry.address_owner = props.operation.input.address_owner[0];
  }

  if (!isListMode.value) {
    fetchDetails();
  }

  formValue.value = entry;
});
</script>
