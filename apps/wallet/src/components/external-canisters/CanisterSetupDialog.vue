<template>
  <VDialog
    v-bind="$attrs"
    v-model="open"
    :persistent="!canClose"
    transition="dialog-bottom-transition"
    scrollable
    :max-width="props.dialogMaxWidth"
  >
    <DataLoader :load="load" @loading="loading = $event">
      <template #error="{ errorMsg, errorDetails }">
        <ErrorCard
          data-test-id="canister-setup-error-card"
          :title="$t('pages.external_canisters.edit_canister_title')"
          :error="errorMsg"
          :error-details="errorDetails"
        />
      </template>
      <VCard data-test-id="canister-setup-ok-card">
        <VToolbar color="background">
          <VToolbarTitle>
            {{
              props.canisterId
                ? $t('pages.external_canisters.edit_canister_title')
                : $t('pages.external_canisters.add_new_canister_title')
            }}
          </VToolbarTitle>
          <VBtn :disabled="!canClose" :icon="mdiClose" @click="open = false" />
        </VToolbar>
        <VDivider />

        todo_ec_config_wizard

        <VCardText v-if="loading" class="py-8">
          <LoadingMessage />
        </VCardText>
      </VCard>
    </DataLoader>
  </VDialog>
</template>
<script lang="ts" setup>
import { Principal } from '@dfinity/principal';
import { mdiClose } from '@mdi/js';
import { computed, ref } from 'vue';
import {
  VBtn,
  VCard,
  VCardText,
  VDialog,
  VDivider,
  VToolbar,
  VToolbarTitle,
} from 'vuetify/components';
import DataLoader from '~/components/DataLoader.vue';
import LoadingMessage from '~/components/LoadingMessage.vue';
import ErrorCard from '~/components/ui/ErrorCard.vue';
import { useStationStore } from '~/stores/station.store';

const props = withDefaults(
  defineProps<{
    canisterId?: Principal;
    open?: boolean;
    dialogMaxWidth?: number;
    readonly?: boolean;
  }>(),
  {
    canisterId: undefined,
    open: false,
    dialogMaxWidth: 800,
    readonly: false,
  },
);

const emit = defineEmits<{
  (event: 'update:open', payload: boolean): void;
}>();

const loading = ref(false);
const submitting = ref(false);
const station = useStationStore();
const canClose = computed(() => !loading.value && !submitting.value);
const open = computed({
  get: () => props.open,
  set: value => emit('update:open', value),
});

const load = async () => {
  if (props.canisterId) {
    await station.service.getExternalCanisterByCanisterId(props.canisterId);

    // TODO: Handle canister details
    return;
  }

  throw new Error('Not implemented');
};
</script>
