<template>
  <div v-if="isIds" class="d-flex ga-4 flex-column">
    <VAlert type="warning" density="compact" variant="tonal">
      {{ $t('request_policies.unsupported_specifier') }}
    </VAlert>
  </div>
</template>
<script setup lang="ts">
import { computed, toRefs } from 'vue';
import { ResourceIds } from '~/generated/station/station.did';
import { variantIs } from '~/utils/helper.utils';

const input = withDefaults(
  defineProps<{
    modelValue?: ResourceIds;
    disabled?: boolean;
    readonly?: boolean;
  }>(),
  {
    modelValue: () => ({ Any: null }),
    disabled: false,
    readonly: false,
  },
);

const props = toRefs(input);

const isIds = computed(() => variantIs(props.modelValue.value, 'Ids'));
</script>
