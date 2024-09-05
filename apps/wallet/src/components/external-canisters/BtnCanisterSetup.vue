<template>
  <VBtn
    data-test-id="btn-canister-setup"
    v-bind="$attrs"
    :size="props.size"
    :variant="props.variant"
    :icon="props.icon && !props.text"
    :color="props.color"
    @click="open = true"
  >
    <VIcon v-if="props.icon" class="mr-1" :icon="props.icon" />
    <slot name="default">
      <span v-if="props.text">{{ props.text }}</span>
    </slot>
    <VIcon v-if="props.appendIcon" class="ml-1" :icon="props.appendIcon" />
  </VBtn>

  <CanisterSetupDialog
    data-test-id="canister-setup-dialog"
    :open="open"
    :canister-id="props.canisterId"
    :readonly="props.readonly"
    :dialog-max-width="800"
    @update:open="
      openEvent => {
        open = openEvent;

        emit('opened', openEvent);
      }
    "
  />
</template>
<script lang="ts" setup>
import { ref } from 'vue';
import { VBtn } from 'vuetify/components';
import CanisterSetupDialog from './CanisterSetupDialog.vue';
import { Principal } from '@dfinity/principal';

const props = withDefaults(
  defineProps<{
    canisterId?: Principal;
    icon?: string;
    text?: string;
    size?: 'x-small' | 'small' | 'default' | 'medium' | 'large' | 'x-large';
    variant?: 'flat' | 'text' | 'outlined' | 'tonal' | 'elevated';
    color?: string;
    readonly?: boolean;
    appendIcon?: string;
  }>(),
  {
    canisterId: undefined,
    icon: undefined,
    text: undefined,
    size: 'default',
    variant: 'elevated',
    color: 'primary',
    readonly: false,
    appendIcon: undefined,
  },
);

const open = ref(false);

const emit = defineEmits<{
  (event: 'opened', payload: boolean): void;
}>();
</script>
