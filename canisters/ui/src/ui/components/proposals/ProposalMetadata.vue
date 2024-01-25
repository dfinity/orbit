<template>
  <div class="d-flex ga-1 flex-row flex-wrap" v-bind="$attrs">
    <VTooltip location="bottom">
      <template #activator="{ props: tooltipProps }">
        <VChip
          size="small"
          v-bind="tooltipProps"
          :prepend-icon="mdiAccountCircle"
          class="flex-grow-1"
          data-testid="proposed_by"
        >
          <span class="text-no-wrap text-truncate" style="max-width: 80px">
            {{
              props.proposal.info.proposer_name?.[0]
                ? props.proposal.info.proposer_name[0]
                : props.proposal.proposed_by
            }}
          </span>
        </VChip>
      </template>
      <span>
        {{
          $t('proposals.proposed_by', {
            name: props.proposal.info.proposer_name?.[0]
              ? props.proposal.info.proposer_name[0]
              : '-',
          })
        }}
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
          data-testid="creation-dt"
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
          data-testid="expiration-dt"
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
import { mdiClockPlusOutline, mdiClockTimeFourOutline, mdiAccountCircle } from '@mdi/js';
import { computed } from 'vue';
import { variantIs } from '~/core';
import { Proposal } from '~/generated/wallet/wallet.did';

const props = defineProps<{
  proposal: Proposal;
}>();

const createdAt = computed<Date>(() => new Date(props.proposal.created_at));
const expiredAt = computed<Date>(() => new Date(props.proposal.expiration_dt));
const isPending = computed<boolean>(() => variantIs(props.proposal.status, 'Created'));
</script>
