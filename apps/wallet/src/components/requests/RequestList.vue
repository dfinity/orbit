<template>
  <div class="container">
    <VProgressLinear v-if="props.loading" indeterminate color="primary" data-test-id="loading" />
    <div v-else :class="{ grid: props.mode === 'grid', list: props.mode === 'list' }">
      <RequestListItem
        v-for="request in props.requests"
        :key="request.id"
        :request="request"
        :details="getDetails(request)"
        :mode="props.mode"
        :show-title="props.showItemsTitle"
        @approved="emit('approved', request)"
        @opened="emit('opened', request)"
        @closed="emit('closed', request)"
      />
      <div
        v-if="!props.requests.length && !props.hideNotFound"
        class="d-block"
        data-test-id="requests-empty-list"
      >
        {{ notFoundText }}
      </div>
    </div>
  </div>
</template>
<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { VProgressLinear } from 'vuetify/components';
import {
  Request,
  RequestAdditionalInfo,
  RequestCallerPrivileges,
} from '~/generated/station/station.did';
import { RequestDetails } from '~/types/station.types';
import RequestListItem from './RequestListItem.vue';

const props = withDefaults(
  defineProps<{
    requests: Request[];
    privileges?: RequestCallerPrivileges[];
    additionals?: RequestAdditionalInfo[];
    hideHeaders?: boolean;
    notFoundText?: string;
    loading?: boolean;
    hideNotFound?: boolean;
    mode?: 'list' | 'grid';
    showItemsTitle?: boolean;
  }>(),
  {
    hideHeaders: false,
    notFoundText: undefined,
    privileges: () => [],
    additionals: () => [],
    loading: false,
    hideNotFound: false,
    mode: 'list',
    showItemsTitle: true,
  },
);

const emit = defineEmits<{
  (event: 'approved', payload: Request): void;
  (event: 'opened', payload: Request): void;
  (event: 'closed', payload: Request): void;
}>();

const i18n = useI18n();
const notFoundText = computed(() => props.notFoundText || i18n.t('requests.no_results_found'));

const getDetails = (request: Request): RequestDetails => {
  const privileges = props.privileges.find(privilege => privilege.id === request.id);
  const info = props.additionals.find(additional => additional.id === request.id);

  return {
    can_approve: !!privileges?.can_approve,
    requester_name: info?.requester_name ?? '',
    approvers: info?.approvers ?? [],
  };
};
</script>

<style scoped>
.container {
  container-type: inline-size;
}

.list {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}
.grid {
  display: grid;
  grid-template-columns: 1fr;
  gap: 1rem;
}

@container (max-width: 799px) or ((min-width: 1200px) and (max-width: 1799px)) {
  .grid > *:nth-child(n + 4) {
    display: none;
  }
}
@container (min-width: 800px) and (max-width: 1199px) {
  .grid {
    grid-template-columns: 1fr 1fr;
  }

  .grid > *:nth-child(2n + 1) {
    border-right: 1px solid rgba(255, 255, 255, var(--v-border-opacity));
  }
}

@container (min-width: 1200px) and (max-width: 1799px) {
  .grid {
    grid-template-columns: 1fr 1fr 1fr;
  }
  .grid > *:not(:nth-child(3)) {
    border-right: 1px solid rgba(255, 255, 255, var(--v-border-opacity));
  }
}

@container (min-width: 1801px) {
  .grid {
    grid-template-columns: 1fr 1fr 1fr 1fr;
  }
  .grid > *:not(:nth-child(4)) {
    border-right: 1px solid rgba(255, 255, 255, var(--v-border-opacity));
  }
}
</style>
