<template>
  <div class="d-flex flex-nowrap">
    <VChip
      v-bind="$attrs"
      :size="props.size"
      :class="props.class"
      :color="status.color"
      variant="flat"
      class="flex-grow-1"
      data-test-id="request-status-chip"
    >
      <slot>
        {{ $t(`requests.status.${status.name}`) }}
      </slot>
    </VChip>
    <template v-if="variantIs(props.status, 'Completed')">
      <VTooltip
        v-model="openDetails"
        location="bottom"
        :open-on-hover="false"
        :open-on-click="true"
        @click:outside="openDetails = false"
      >
        <template #activator="{ props: infoProps }">
          <VBtn :icon="mdiInformationOutline" size="x-small" v-bind="infoProps" />
        </template>
        {{ $t('requests.processing_completed_at', { dt: props.status.Completed.completed_at }) }}
      </VTooltip>
    </template>
    <template v-else-if="variantIs(props.status, 'Failed')">
      <VTooltip
        v-model="openDetails"
        location="bottom"
        :open-on-hover="false"
        :open-on-click="true"
        @click:outside="openDetails = false"
      >
        <template #activator="{ props: infoProps }">
          <VBtn :icon="mdiInformationOutline" size="x-small" v-bind="infoProps" />
        </template>
        {{
          props.status.Failed.reason?.[0]
            ? props.status.Failed.reason[0]
            : $t('requests.no_failed_reason')
        }}
      </VTooltip>
    </template>
    <template v-else-if="variantIs(props.status, 'Cancelled')">
      <VTooltip
        v-model="openDetails"
        location="bottom"
        :open-on-hover="false"
        :open-on-click="true"
        @click:outside="openDetails = false"
      >
        <template #activator="{ props: infoProps }">
          <VBtn :icon="mdiInformationOutline" size="x-small" v-bind="infoProps" />
        </template>
        {{
          props.status.Cancelled.reason?.[0]
            ? props.status.Cancelled.reason[0]
            : $t('requests.no_cancelled_reason')
        }}
      </VTooltip>
    </template>
    <template v-else-if="variantIs(props.status, 'Processing')">
      <VTooltip
        v-model="openDetails"
        location="bottom"
        :open-on-hover="false"
        :open-on-click="true"
        @click:outside="openDetails = false"
      >
        <template #activator="{ props: infoProps }">
          <VBtn :icon="mdiInformationOutline" size="x-small" v-bind="infoProps" />
        </template>
        {{ $t('requests.processing_started_at', { dt: props.status.Processing.started_at }) }}
      </VTooltip>
    </template>
    <template v-else-if="variantIs(props.status, 'Scheduled')">
      <VTooltip
        v-model="openDetails"
        location="bottom"
        :open-on-hover="false"
        :open-on-click="true"
        @click:outside="openDetails = false"
      >
        <template #activator="{ props: infoProps }">
          <VBtn :icon="mdiInformationOutline" size="x-small" v-bind="infoProps" />
        </template>
        {{ $t('requests.processing_scheduled_at', { dt: props.status.Scheduled.scheduled_at }) }}
      </VTooltip>
    </template>
  </div>
</template>

<script setup lang="ts">
import { mdiInformationOutline } from '@mdi/js';
import { computed, ref } from 'vue';
import { RequestStatus } from '~/generated/station/station.did';
import { variantIs } from '~/utils/helper.utils';

const props = withDefaults(
  defineProps<{
    status: RequestStatus;
    class?: string;
    size?: string | number;
  }>(),
  {
    class: 'text-lowercase',
    size: 'default',
  },
);

let openDetails = ref(false);

const status = computed(() => {
  if (variantIs(props.status, 'Created')) {
    return { name: 'created', color: 'warning' };
  }

  if (variantIs(props.status, 'Cancelled')) {
    return { name: 'cancelled', color: 'error' };
  }

  if (variantIs(props.status, 'Approved')) {
    return { name: 'approved', color: 'info' };
  }

  if (variantIs(props.status, 'Rejected')) {
    return { name: 'rejected', color: 'error' };
  }

  if (variantIs(props.status, 'Completed')) {
    return { name: 'completed', color: 'success' };
  }

  if (variantIs(props.status, 'Failed')) {
    return { name: 'failed', color: 'error' };
  }

  if (variantIs(props.status, 'Processing')) {
    return { name: 'processing', color: 'info' };
  }

  if (variantIs(props.status, 'Scheduled')) {
    return { name: 'scheduled', color: 'info' };
  }

  return { name: 'unknown', color: 'default' };
});
</script>
