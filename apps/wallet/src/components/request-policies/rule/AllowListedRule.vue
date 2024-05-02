<template>
  <div class="d-flex align-center justify-start">
    {{ $t('request_policies.rule.allowlisted') }}
    <VBtn
      v-if="!props.disabled.value"
      :icon="mdiTrashCanOutline"
      variant="flat"
      size="small"
      color="transparent"
      density="compact"
      class="ml-2"
      @click="emit('remove')"
    />
  </div>
</template>

<script setup lang="ts">
import { mdiTrashCanOutline } from '@mdi/js';
import { toRefs } from 'vue';

const input = withDefaults(
  defineProps<{
    disabled?: boolean;
  }>(),
  {
    disabled: false,
  },
);

const props = toRefs(input);

const emit = defineEmits<{
  (event: 'remove', payload: void): void;
}>();
</script>
