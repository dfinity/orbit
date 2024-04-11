<template>
  <VBtn
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

  <AccountSetupDialog
    :open="open"
    :account-id="props.accountId"
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
import AccountSetupDialog from '~/components/accounts/AccountSetupDialog.vue';
import { UUID } from '~/generated/wallet/wallet.did';

const props = withDefaults(
  defineProps<{
    accountId?: UUID;
    icon?: string;
    text?: string;
    size?: 'x-small' | 'small' | 'default' | 'medium' | 'large' | 'x-large';
    variant?: 'flat' | 'text' | 'outlined';
    color?: string;
    readonly?: boolean;
    appendIcon?: string;
  }>(),
  {
    accountId: undefined,
    icon: undefined,
    text: undefined,
    size: 'default',
    variant: 'flat',
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
