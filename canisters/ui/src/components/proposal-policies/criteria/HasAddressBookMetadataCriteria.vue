<template>
  <div class="d-flex align-center justify-start">
    {{ $t('proposal_policies.criteria.hasaddressbookmetadata') }}
    <VBtn
      :icon="mdiTrashCanOutline"
      variant="flat"
      size="small"
      color="transparent"
      density="compact"
      class="ml-2"
      @click="emit('remove')"
    />
  </div>
  <div class="d-flex flex-row ga-2">
    <VTextField
      v-model="model.key"
      :label="$t('terms.key')"
      :rules="[requiredRule]"
      variant="underlined"
      density="comfortable"
    />
    <VTextField
      v-model="model.value"
      :label="$t('terms.value')"
      variant="underlined"
      density="comfortable"
    />
  </div>
</template>

<script setup lang="ts">
import { mdiTrashCanOutline } from '@mdi/js';
import { computed } from 'vue';
import { toRefs } from 'vue';
import { AddressBookMetadata } from '~/generated/wallet/wallet.did';
import { requiredRule } from '~/utils/form.utils';

const input = withDefaults(
  defineProps<{
    modelValue: AddressBookMetadata;
  }>(),
  {},
);

const props = toRefs(input);

const model = computed({
  get: () => props.modelValue.value,
  set: value => emit('update:modelValue', value),
});

const emit = defineEmits<{
  (event: 'update:modelValue', payload: AddressBookMetadata): void;
  (event: 'remove', payload: void): void;
}>();
</script>
