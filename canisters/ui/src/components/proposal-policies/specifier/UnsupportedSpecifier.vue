<template>
  <div v-if="isId || isGroup" class="d-flex ga-4 flex-column">
    <VAlert type="warning" density="compact" variant="tonal">
      {{ $t('proposal_policies.unsupported_specifier') }}
    </VAlert>
  </div>
</template>
<script setup lang="ts">
import { computed, toRefs } from 'vue';
import { CommonSpecifier } from '~/generated/wallet/wallet.did';
import { variantIs } from '~/utils/helper.utils';

const input = withDefaults(
  defineProps<{
    modelValue?: CommonSpecifier;
    disabled?: boolean;
  }>(),
  {
    modelValue: () => ({ Any: null }),
    disabled: false,
  },
);

const props = toRefs(input);

const isId = computed(() => variantIs(props.modelValue.value, 'Id'));
const isGroup = computed(() => variantIs(props.modelValue.value, 'Group'));
</script>
