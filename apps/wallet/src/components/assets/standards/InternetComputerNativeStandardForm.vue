<template>
  <DiffView :before-value="currentLedgerId" :after-value="ledgerId">
    <template #default="{ value, diffMode }">
      <VTextField
        :model-value="value"
        :name="
          diffMode === 'before'
            ? 'metadata_ledger_canister_id-before'
            : 'metadata_ledger_canister_id'
        "
        :label="$t('pages.assets.forms.ledger_canister_id')"
        :variant="props.readonly ? 'plain' : 'filled'"
        density="comfortable"
        :readonly="props.readonly || diffMode === 'before'"
        :prepend-icon="mdiDatabase"
        :rules="diffMode === 'before' ? [] : [requiredRule, validCanisterId]"
        @update:model-value="val => diffMode === 'after' && (ledgerId = val)"
      />
    </template>
  </DiffView>
  <DiffView :before-value="currentIndexId" :after-value="indexId">
    <template #default="{ value, diffMode }">
      <VTextField
        :model-value="value"
        :name="
          diffMode === 'before' ? 'metadata_index_canister_id-before' : 'metadata_index_canister_id'
        "
        :label="$t('pages.assets.forms.index_canister_id')"
        :variant="props.readonly ? 'plain' : 'filled'"
        density="comfortable"
        :readonly="props.readonly || diffMode === 'before'"
        :prepend-icon="mdiDatabase"
        :rules="diffMode === 'before' ? [] : [requiredRule, validCanisterId]"
        @update:model-value="val => diffMode === 'after' && (indexId = val)"
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
