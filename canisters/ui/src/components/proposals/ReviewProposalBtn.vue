<template>
  <VBtn
    v-bind="$attrs"
    data-test-id="review-proposal-btn"
    :size="props.size"
    :variant="props.variant"
    :icon="props.icon && !btnText"
    :color="props.color"
    @click="open = true"
  >
    <slot name="default">
      {{ btnText }}
    </slot>
  </VBtn>

  <ProposalDialog v-model:open="open" :proposal-id="props.proposal.id" @voted="emit('voted')" />
</template>
<script setup lang="ts">
import { computed, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import ProposalDialog from '~/components/proposals/ProposalDialog.vue';
import { Proposal } from '~/generated/wallet/wallet.did';
import { ProposalDetails } from '~/types/wallet.types';

const props = withDefaults(
  defineProps<{
    proposal: Proposal;
    details: ProposalDetails;
    icon?: string;
    text?: string;
    size?: 'x-small' | 'small' | 'default' | 'medium' | 'large' | 'x-large';
    variant?: 'flat' | 'text' | 'outlined';
    color?: string;
    readonly?: boolean;
  }>(),
  {
    icon: undefined,
    text: undefined,
    size: 'small',
    variant: 'flat',
    color: 'default',
    readonly: false,
  },
);

const emit = defineEmits<{
  (event: 'voted'): void;
  (event: 'closed'): void;
  (event: 'opened'): void;
}>();

const open = ref(false);
const i18n = useI18n();
const btnText = computed(
  () => props.text || (props.details?.can_vote ? i18n.t('terms.review') : i18n.t('terms.view')),
);

watch(
  () => open.value,
  open => {
    if (open) {
      emit('opened');
    } else {
      emit('closed');
    }
  },
);
</script>
