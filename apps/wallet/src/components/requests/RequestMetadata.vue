<template>
  <div class="d-flex ga-1 flex-row flex-wrap" v-bind="$attrs">
    <VTooltip location="bottom">
      <template #activator="{ props: tooltipProps }">
        <VChip
          size="small"
          v-bind="tooltipProps"
          :prepend-icon="mdiAccountCircle"
          class="flex-grow-1"
          data-test-id="requested_by"
        >
          <span class="text-no-wrap text-truncate" style="max-width: 80px">
            {{ props.details.requester_name }}
          </span>
        </VChip>
      </template>
      <span>
        {{ $t('requests.requested_by', { name: props.details.requester_name }) }}
        <br />
        {{ $t('requests.requester_id', { id: props.request.requested_by }) }}
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
import { Request } from '~/generated/station/station.did';
import type { RequestDetails } from '~/types/station.types';
import { variantIs } from '~/utils/helper.utils';

const props = defineProps<{
  request: Request;
  details: RequestDetails;
}>();

const createdAt = computed<Date>(() => new Date(props.request.created_at));
const expiredAt = computed<Date>(() => new Date(props.request.expiration_dt));
const isPending = computed<boolean>(() => variantIs(props.request.status, 'Created'));
</script>
