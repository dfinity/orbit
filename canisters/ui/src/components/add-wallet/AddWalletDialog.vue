<template>
  <VDialog width="600px" :persistent="persist" @update:model-value="onVisibilityChange">
    <template #activator="props">
      <slot name="activator" v-bind="props"></slot>
    </template>
    <template #default="{ isActive }">
      <AddWalletForm
        ref="form"
        @cancelled="isActive.value = false"
        @submitted="isActive.value = false"
        @working="isWorking => (persist = isWorking)"
      />
    </template>
  </VDialog>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import AddWalletForm from './AddWalletForm.vue';

const persist = ref(false);
const form = ref<InstanceType<typeof AddWalletForm> | null>(null);

function onVisibilityChange(visible: boolean) {
  if (visible) {
    form.value?.reset();
  }
}
</script>
