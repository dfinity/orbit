<template>
  <ProposalDialog
    v-if="proposalId"
    v-model:open="open"
    :proposal-id="proposalId"
    @voted="open = false"
  />
</template>
<script setup lang="ts">
import { ref, watch } from 'vue';
import { useRouter } from 'vue-router';
import ProposalDialog from '~/components/proposals/ProposalDialog.vue';
import { PROPOSAL_DIALOG_QUERY_PARAM } from '~/core/constants.core';
import { isValidUUID } from '~/utils/helper.utils';

const open = ref(false);
const router = useRouter();
const proposalId = ref<string | null>(null);

watch(
  () => router.currentRoute.value,
  route => {
    const queryParam = route.query?.[PROPOSAL_DIALOG_QUERY_PARAM];
    if (!queryParam) {
      proposalId.value = null;
    }

    const id = Array.isArray(queryParam) ? queryParam?.[0] ?? '' : queryParam;

    proposalId.value = isValidUUID(`${id}`) ? id : null;
  },
  { deep: true, immediate: true },
);

watch(
  () => proposalId.value,
  () => {
    open.value = !!proposalId.value;
  },
  { immediate: true },
);

watch(
  () => open.value,
  open => {
    if (!open) {
      // Delay to allow the dialog to close before removing the query param
      setTimeout(() => {
        const query = Object.assign({}, router.currentRoute.value.query);
        if (query[PROPOSAL_DIALOG_QUERY_PARAM] !== undefined) {
          delete query[PROPOSAL_DIALOG_QUERY_PARAM];
        }

        router.replace({ query });
      }, 100);
    }
  },
);
</script>
