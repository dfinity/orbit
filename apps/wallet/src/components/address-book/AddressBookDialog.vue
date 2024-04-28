<template>
  <VDialog
    v-model="openModel"
    :persistent="loading || saving"
    transition="dialog-bottom-transition"
    scrollable
    :max-width="props.dialogMaxWidth.value"
  >
    <DataLoader
      v-slot="{ data }"
      :load="loadAddressBookEntry"
      @loading="loading = $event"
      @loaded="addressBookEntry = $event.entry"
    >
      <VCard>
        <VToolbar color="background">
          <VToolbarTitle>{{ $t('app.address_book_entry') }}</VToolbarTitle>
          <VBtn :disabled="loading || saving" :icon="mdiClose" @click="openModel = false" />
        </VToolbar>
        <VCardText v-if="loading" class="py-8">
          <LoadingMessage />
        </VCardText>
        <VCardText v-else>
          <AddressBookForm
            v-if="data"
            v-model="addressBookEntry"
            v-model:trigger-submit="triggerSubmit"
            :display="{
              id: true,
            }"
            :disabled="props.readonly.value"
            @submit="save"
            @valid="valid = $event"
          />
        </VCardText>
        <VDivider />
        <VCardActions class="pa-3">
          <VSpacer />
          <VBtn
            v-if="!props.readonly.value"
            :disabled="!canSave"
            :loading="saving"
            color="primary"
            variant="elevated"
            @click="triggerSubmit = true"
          >
            {{ props.addressBookEntryId.value ? $t('terms.save') : $t('terms.create') }}
          </VBtn>
        </VCardActions>
      </VCard>
    </DataLoader>
  </VDialog>
</template>
<script lang="ts" setup>
import { mdiClose } from '@mdi/js';
import { computed, ref, toRefs } from 'vue';
import {
  VBtn,
  VCard,
  VCardActions,
  VCardText,
  VDialog,
  VDivider,
  VSpacer,
  VToolbar,
  VToolbarTitle,
} from 'vuetify/components';
import DataLoader from '~/components/DataLoader.vue';
import LoadingMessage from '~/components/LoadingMessage.vue';
import AddressBookForm from '~/components/address-book/AddressBookForm.vue';
import {
  useOnFailedOperation,
  useOnSuccessfulOperation,
} from '~/composables/notifications.composable';
import logger from '~/core/logger.core';
import { AddressBookEntry, UUID } from '~/generated/station/station.did';
import { useStationStore } from '~/stores/station.store';
import { BlockchainStandard } from '~/types/chain.types';
import { assertAndReturn } from '~/utils/helper.utils';

const input = withDefaults(
  defineProps<{
    addressBookEntryId?: UUID;
    open?: boolean;
    dialogMaxWidth?: number;
    readonly?: boolean;
  }>(),
  {
    addressBookEntryId: undefined,
    open: false,
    dialogMaxWidth: 800,
    readonly: false,
  },
);

const emit = defineEmits<{
  (event: 'update:open', payload: boolean): void;
}>();

const props = toRefs(input);
const valid = ref(true);
const loading = ref(false);
const saving = ref(false);
const addressBookEntry = ref<Partial<AddressBookEntry>>({});
const openModel = computed({
  get: () => props.open.value,
  set: value => emit('update:open', value),
});

const station = useStationStore();

const loadAddressBookEntry = async (): Promise<{
  entry: Partial<AddressBookEntry>;
}> => {
  if (props.addressBookEntryId.value === undefined) {
    const createModel: Partial<AddressBookEntry> = {
      standard: BlockchainStandard.Native,
    };

    return { entry: createModel };
  }

  const result = await station.service.getAddressBookEntry(
    {
      address_book_entry_id: props.addressBookEntryId.value,
    },
    true,
  );
  return { entry: result.address_book_entry };
};

const canSave = computed(() => {
  return valid.value && !loading.value;
});

const triggerSubmit = ref(false);

const save = async (): Promise<void> => {
  if (!canSave.value) {
    return;
  }

  try {
    saving.value = true;
    if (addressBookEntry.value.id) {
      const request = await station.service.editAddressBookEntry({
        address_book_entry_id: addressBookEntry.value.id,
        address_owner: [assertAndReturn(addressBookEntry.value.address_owner)],
        change_metadata: [
          {
            ReplaceAllBy: addressBookEntry.value.metadata ?? [],
          },
        ],
      });

      useOnSuccessfulOperation(request);

      openModel.value = false;
      return;
    }

    const request = await station.service.addAddressBookEntry({
      blockchain: assertAndReturn(addressBookEntry.value.blockchain, 'blockchain'),
      standard: assertAndReturn(addressBookEntry.value.standard, 'standard'),
      address_owner: assertAndReturn(addressBookEntry.value.address_owner, 'address_owner'),
      address: assertAndReturn(addressBookEntry.value.address, 'address'),
      metadata: addressBookEntry.value.metadata ?? [],
    });

    useOnSuccessfulOperation(request);

    openModel.value = false;
  } catch (error) {
    logger.error(`Failed to save address book entry ${error}`);

    useOnFailedOperation();
  } finally {
    saving.value = false;
  }
};
</script>
