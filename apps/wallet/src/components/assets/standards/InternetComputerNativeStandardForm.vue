<template>
  <DiffView :before-value="currentLedgerId" :after-value="ledgerId">
    <template #default="{ value, mode }">
      <VTextField
        :model-value="value"
        @update:model-value="val => mode === 'after' && (ledgerId = val)"
        :name="
          mode === 'before' ? 'metadata_ledger_canister_id-before' : 'metadata_ledger_canister_id'
        "
        :label="$t('pages.assets.forms.ledger_canister_id')"
        variant="filled"
        density="comfortable"
        :disabled="props.readonly || mode === 'before'"
        :prepend-icon="mdiDatabase"
        :rules="mode === 'before' ? [] : [requiredRule, validCanisterId]"
      />
    </template>
  </DiffView>
  <DiffView :before-value="currentIndexId" :after-value="indexId">
    <template #default="{ value, mode }">
      <VTextField
        :model-value="value"
        @update:model-value="val => mode === 'after' && (indexId = val)"
        :name="
          mode === 'before' ? 'metadata_index_canister_id-before' : 'metadata_index_canister_id'
        "
        :label="$t('pages.assets.forms.index_canister_id')"
        variant="filled"
        density="comfortable"
        :disabled="props.readonly || mode === 'before'"
        :prepend-icon="mdiDatabase"
        :rules="mode === 'before' ? [] : [requiredRule, validCanisterId]"
      />
    </template>
  </DiffView>
</template>
<script lang="ts" setup>
import { mdiDatabase } from '@mdi/js';
import { computed } from 'vue';
import { VTextField } from 'vuetify/components';
import DiffView from '~/components/requests/DiffView.vue';
import { AssetMetadata } from '~/generated/station/station.did';
import { requiredRule, validCanisterId } from '~/utils/form.utils';

const props = defineProps<{
  modelValue: AssetMetadata[];
  currentMetadata?: AssetMetadata[];
  readonly: boolean;
}>();

const emit = defineEmits<{
  'update:modelValue': [AssetMetadata[]];
}>();

const ledgerId = computed<string | undefined>({
  get: () => props.modelValue.find(m => m.key === 'ledger_canister_id')?.value,
  set: (value: string | undefined) => {
    const newValue = props.modelValue.filter(m => m.key !== 'ledger_canister_id');
    if (value) {
      newValue.push({ key: 'ledger_canister_id', value });
    }
    emit('update:modelValue', newValue);
  },
});

const indexId = computed<string | undefined>({
  get: () => props.modelValue.find(m => m.key === 'index_canister_id')?.value,
  set: (value: string | undefined) => {
    const newValue = props.modelValue.filter(m => m.key !== 'index_canister_id');
    if (value) {
      newValue.push({ key: 'index_canister_id', value });
    }
    emit('update:modelValue', newValue);
  },
});

const currentLedgerId = computed<string | undefined>(
  () => props.currentMetadata?.find(m => m.key === 'ledger_canister_id')?.value,
);
const currentIndexId = computed<string | undefined>(
  () => props.currentMetadata?.find(m => m.key === 'index_canister_id')?.value,
);
</script>
