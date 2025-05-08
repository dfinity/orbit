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
        <span class="text-break">
          {{ formValue.address }}
        </span>
      </template>
    </RequestOperationListRow>
  </div>
  <VProgressCircular v-else-if="loading" indeterminate />
  <template v-else>
    <VAlert v-if="currentEntryFailed" type="error" variant="tonal" density="compact" class="mb-4">
      {{ $t('requests.failed_to_fetch_details') }}
      <div>{{ currentEntryFailed }}</div>
    </VAlert>
    <AddressBookForm :model-value="formValue" mode="view" :current-entry="currentEntry" />
  </template>
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
import { useAppStore } from '~/stores/app.store';
import { getErrorMessage } from '~/utils/error.utils';

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
const isDiffMode = computed(() => !isListMode.value && variantIs(props.request.status, 'Created'));
const formValue: Ref<Partial<AddressBookEntry>> = ref({});
const loading = ref(false);
const station = useStationStore();
const appStore = useAppStore();
const currentEntry = ref<AddressBookEntry | undefined>(undefined);
const currentEntryFailed = ref<string | undefined>();

const fetchDetails = async () => {
  try {
    loading.value = true;
    currentEntryFailed.value = undefined;
    const response = await station.service.getAddressBookEntry(
      {
        address_book_entry_id: props.operation.input.address_book_entry_id,
      },
      true,
    );

    if (isDiffMode.value) {
      currentEntry.value = response.address_book_entry;
    }

    let currentMetadata = response.address_book_entry.metadata;
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

    formValue.value.address = response.address_book_entry.address;
    formValue.value.blockchain = response.address_book_entry.blockchain;
  } catch (e) {
    logger.error('Failed to fetch address book entry details', e);
    if (isDiffMode.value) {
      currentEntryFailed.value = getErrorMessage(e);
    }
    appStore.sendErrorNotification(e);
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
