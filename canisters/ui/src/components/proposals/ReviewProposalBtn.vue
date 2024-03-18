<template>
  <VBtn
    v-bind="$attrs"
    data-test-id="review-proposal-btn"
    :size="props.size"
    :variant="props.variant"
    :icon="props.icon && !btnText"
    :color="props.color"
    @click="openDialog"
  >
    <slot name="default">
      {{ btnText }}
    </slot>
  </VBtn>
</template>
<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { useProposalOverlay } from '~/composables/proposal.composable';
import { UUID } from '~/generated/wallet/wallet.did';

const proposalOverlay = useProposalOverlay();

const props = withDefaults(
  defineProps<{
    proposalId: UUID;
    canVote: boolean;
    icon?: string;
    text?: string;
    size?: 'x-small' | 'small' | 'default' | 'medium' | 'large' | 'x-large';
    variant?: 'flat' | 'text' | 'outlined';
    color?: string;
    readonly?: boolean;
  }>(),
  {
    proposal: undefined,
    canVote: false,
    icon: undefined,
    text: undefined,
    size: 'small',
    variant: 'flat',
    color: 'default',
    readonly: false,
  },
);

defineEmits<{
  (event: 'voted'): void;
  (event: 'closed'): void;
  (event: 'opened'): void;
}>();

const i18n = useI18n();
const btnText = computed(
  () => props.text || (props.canVote ? i18n.t('terms.review') : i18n.t('terms.view')),
);

function openDialog() {
  proposalOverlay.open(props.proposalId);
}
</script>
