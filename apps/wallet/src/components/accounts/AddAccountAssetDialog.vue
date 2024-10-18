<template>
  <VDialog
    v-model="openModel"
    :persistent="loading || saving"
    transition="dialog-bottom-transition"
    scrollable
    :max-width="props.dialogMaxWidth.value"
  >
    <VCard :loading="saving" data-test-id="add-asset-dialog-form">
      <VToolbar color="background">
        <VToolbarTitle>{{ $t('pages.account.add_asset') }}</VToolbarTitle>
        <VBtn :disabled="loading || saving" :icon="mdiClose" @click="openModel = false" />
      </VToolbar>
      <VCardText>
        <TokenAutocomplete
          v-model="assets"
          :excluded-ids="props.account.value.assets.map(asset => asset.asset_id)"
          class="mb-2"
          :label="$t('terms.asset')"
          :prepend-icon="mdiBank"
          :rules="[requiredRule]"
          variant="filled"
          density="comfortable"
          :multiple="true"
          :no-data-text="$t('pages.account.no_assets_to_add')"
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
          data-test-id="add-asset-dialog-save-button"
          @click="save"
        >
          {{ $t('terms.submit') }}
        </VBtn>
      </VCardActions>
    </VCard>
  </VDialog>
</template>
<script lang="ts" setup>
import { mdiBank, mdiClose } from '@mdi/js';
import { UUID } from 'crypto';
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
import {
  useOnFailedOperation,
  useOnSuccessfulOperation,
} from '~/composables/notifications.composable';
import logger from '~/core/logger.core';
import { Account } from '~/generated/station/station.did';
import { services } from '~/plugins/services.plugin';
import { requiredRule } from '~/utils/form.utils';
import TokenAutocomplete from '../inputs/TokenAutocomplete.vue';

const input = withDefaults(
  defineProps<{
    account: Account;
    open?: boolean;
    dialogMaxWidth?: number;
    readonly?: boolean;
  }>(),
  {
    open: false,
    dialogMaxWidth: 800,
    readonly: false,
  },
);

const emit = defineEmits<{
  (event: 'update:open', payload: boolean): void;
}>();

const assets = ref<UUID[]>([]);

const props = toRefs(input);
const valid = ref(true);
const loading = ref(false);
const saving = ref(false);
const openModel = computed({
  get: () => props.open.value,
  set: value => emit('update:open', value),
});

const stationService = services().station;

const canSave = computed(() => {
  return valid.value && !loading.value;
});

const save = async (): Promise<void> => {
  if (!canSave.value) {
    return;
  }

  try {
    saving.value = true;

    const newRequest = await stationService.editAccount({
      account_id: props.account.value.id,
      change_assets: [{ Change: { add_assets: assets.value, remove_assets: [] } }],
      configs_permission: [],
      configs_request_policy: [],
      name: [],
      read_permission: [],
      transfer_permission: [],
      transfer_request_policy: [],
    });

    useOnSuccessfulOperation(newRequest);

    openModel.value = false;
  } catch (error) {
    logger.error(`Failed to request adding asset ${error}`);

    useOnFailedOperation();
  } finally {
    saving.value = false;
  }
};
</script>
