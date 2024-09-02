<template>
  <VDialog
    v-model="open"
    :persistent="!canClose"
    transition="dialog-bottom-transition"
    scrollable
    :max-width="props.dialogMaxWidth"
  >
    <DataLoader :load="load" @loading="loading = $event">
      <template #error="{ errorMsg, errorDetails }">
        <ErrorCard
          :title="$t('pages.external_canisters.edit_canister_title')"
          :error="errorMsg"
          :error-details="errorDetails"
        />
      </template>
      <VCard>
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
const canClose = computed(() => !loading.value && !submitting.value);
const open = computed({
  get: () => props.open,
  set: value => emit('update:open', value),
});

const load = async () => {
  // todo_ec_load_canister
};
</script>
