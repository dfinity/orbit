<template>
  <div class="d-flex ga-1 flex-row flex-wrap" v-bind="$attrs">
    <VTooltip location="bottom">
      <template #activator="{ props: tooltipProps }">
        <VChip
          size="small"
          v-bind="tooltipProps"
          :prepend-icon="mdiAccountCircle"
          class="flex-grow-1"
          data-test-id="proposed_by"
        >
          <span class="text-no-wrap text-truncate" style="max-width: 80px">
            {{
              props.details.proposer_name ? props.details.proposer_name : props.proposal.proposed_by
            }}
          </span>
        </VChip>
      </template>
      <span>
        {{ $t('proposals.proposed_by', { name: props.details.proposer_name ?? '-' }) }}
        <br />
        {{ $t('proposals.proposer_id', { id: props.proposal.proposed_by }) }}
      </span>
    </VTooltip>
    <VTooltip location="bottom">
      <template #activator="{ props: tooltipProps }">
        <VChip
          size="small"
          v-bind="tooltipProps"
          :prepend-icon="mdiClockPlusOutline"
          class="flex-grow-1"
          data-test-id="creation-dt"
        >
          {{ createdAt.toLocaleDateString() }}
        </VChip>
      </template>
      <span>
        {{ $t('terms.created_at') }}
        {{ `${createdAt.toLocaleDateString()} ${createdAt.toLocaleTimeString()}` }}
      </span>
    </VTooltip>
    <VTooltip v-if="isPending" location="bottom">
      <template #activator="{ props: tooltipProps }">
        <VChip
          size="small"
          v-bind="tooltipProps"
          :prepend-icon="mdiClockTimeFourOutline"
          color="error"
          class="flex-grow-1"
          data-test-id="expiration-dt"
        >
          {{ $t('terms.expires_at') }}
          {{ expiredAt.toLocaleDateString() }}
        </VChip>
      </template>
      <span>
        {{ $t('terms.expires_at') }}
        {{ `${expiredAt.toLocaleDateString()} ${expiredAt.toLocaleTimeString()}` }}
      </span>
    </VTooltip>
  </div>
</template>
<script setup lang="ts">
import { mdiAccountCircle, mdiClockPlusOutline, mdiClockTimeFourOutline } from '@mdi/js';
import { computed } from 'vue';
import { VChip, VTooltip } from 'vuetify/components';
import { Proposal } from '~/generated/wallet/wallet.did';
import type { ProposalDetails } from '~/types/wallet.types';
import { variantIs } from '~/utils/helper.utils';

const props = defineProps<{
  proposal: Proposal;
  details: ProposalDetails;
}>();

const createdAt = computed<Date>(() => new Date(props.proposal.created_at));
const expiredAt = computed<Date>(() => new Date(props.proposal.expiration_dt));
const isPending = computed<boolean>(() => variantIs(props.proposal.status, 'Created'));
</script>
