<template>
  <VTextField
    v-model="ledgerId"
    name="metadata_ledger_canister_id"
    :label="$t('pages.assets.forms.ledger_canister_id')"
    variant="filled"
    density="comfortable"
    :disabled="props.readonly"
    :prepend-icon="mdiDatabase"
    :rules="[requiredRule, validCanisterId]"
  />
  <VTextField
    v-model="indexId"
    name="metadata_index_canister_id"
    :label="$t('pages.assets.forms.index_canister_id')"
    variant="filled"
    density="comfortable"
    :disabled="props.readonly"
    :prepend-icon="mdiDatabase"
    :rules="[validCanisterId]"
  />
</template>
<script lang="ts" setup>
import { mdiDatabase } from '@mdi/js';
import { computed, onMounted, ref, watch } from 'vue';
import { VTextField } from 'vuetify/components';
import { AssetMetadata } from '~/generated/station/station.did';
import { requiredRule, validCanisterId } from '~/utils/form.utils';

const props = defineProps<{
  modelValue: AssetMetadata[];
  readonly: boolean;
}>();

const emit = defineEmits<{
  'update:modelValue': [AssetMetadata[]];
}>();

const ledgerId = ref();
const indexId = ref();

const model = computed({
  get: () => props.modelValue,
  set: (value: AssetMetadata[]) => {
    emit('update:modelValue', value);
  },
});

onMounted(() => {
  const ledger = props.modelValue.find(m => m.key === 'ledger_canister_id');
  const index = props.modelValue.find(m => m.key === 'index_canister_id');

  ledgerId.value = ledger?.value;
  indexId.value = index?.value;
});

watch(ledgerId, () => {
  const newValue = model.value.filter(m => m.key !== 'ledger_canister_id');
  newValue.push({ key: 'ledger_canister_id', value: ledgerId.value });

  model.value = newValue;
});

watch(indexId, () => {
  const newValue = model.value.filter(m => m.key !== 'index_canister_id');
  newValue.push({ key: 'index_canister_id', value: indexId.value });

  model.value = newValue;
});
</script>
