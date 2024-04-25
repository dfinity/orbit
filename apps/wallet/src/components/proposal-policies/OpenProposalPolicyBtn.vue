<template>
  <VBtn
    :size="props.size.value"
    :variant="props.variant.value"
    :icon="props.icon.value && !props.text.value"
    :color="props.color.value"
    @click="open = true"
  >
    <VIcon v-if="props.icon.value" :icon="props.icon.value" />
    <span v-if="props.text">{{ props.text.value }}</span>
  </VBtn>

  <ProposalPolicyDialog
    :open="open"
    :policy-id="props.policyId.value"
    :readonly="props.readonly.value"
    @update:open="
      openEvent => {
        open = openEvent;

        emit('opened', openEvent);
      }
    "
  />
</template>
<script lang="ts" setup>
import { ref, toRefs } from 'vue';
import { VBtn, VIcon } from 'vuetify/components';
import ProposalPolicyDialog from '~/components/proposal-policies/ProposalPolicyDialog.vue';
import { UUID } from '~/generated/station/station.did';

const input = withDefaults(
  defineProps<{
    policyId?: UUID;
    icon?: string;
    text?: string;
    size?: 'x-small' | 'small' | 'default' | 'medium' | 'large' | 'x-large';
    variant?: 'flat' | 'text' | 'outlined' | 'elevated';
    color?: string;
    readonly?: boolean;
  }>(),
  {
    policyId: undefined,
    icon: undefined,
    text: undefined,
    size: 'default',
    variant: 'elevated',
    color: 'primary',
    readonly: false,
  },
);

const open = ref(false);

const emit = defineEmits<{
  (event: 'opened', payload: boolean): void;
}>();

const props = toRefs(input);
</script>
