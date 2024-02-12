<template>
  <div class="d-flex align-center justify-start">
    {{ $t('proposal_policies.criteria.autoadopted') }}
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
</template>

<script setup lang="ts">
import { mdiTrashCanOutline } from '@mdi/js';

const emit = defineEmits<{
  (event: 'remove', payload: void): void;
}>();
</script>
