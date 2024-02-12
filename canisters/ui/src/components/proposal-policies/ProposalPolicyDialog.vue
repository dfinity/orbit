<template>
  <VDialog
    v-model="openModel"
    :persistent="loading"
    transition="dialog-bottom-transition"
    scrollable
    :max-width="props.dialogMaxWidth.value"
  >
    <DataLoader v-slot="{ data }" :load="loadPolicy" @loading="loading = $event">
      <VCard :loading="loading">
        <VToolbar dark color="surface">
          <VToolbarTitle>{{ $t('pages.proposal_policies.dialog_title') }}</VToolbarTitle>
          <VBtn :disabled="loading" :icon="mdiClose" dark @click="openModel = false" />
        </VToolbar>
        <VCardText>
          <ProposalPolicyForm v-if="data" v-model="data.policy" />
        </VCardText>
        <VCardActions class="pa-3">
          <VSpacer />
          <VBtn>{{ $t('terms.save') }}</VBtn>
        </VCardActions>
      </VCard>
    </DataLoader>
  </VDialog>
</template>
<script lang="ts" setup>
import { mdiClose } from '@mdi/js';
import { computed, toRefs, ref } from 'vue';
import DataLoader from '~/components/DataLoader.vue';
import ProposalPolicyForm from '~/components/proposal-policies/ProposalPolicyForm.vue';
import { ProposalPolicy, UUID } from '~/generated/wallet/wallet.did';
import { useWalletStore } from '~/stores/wallet.store';

const input = withDefaults(
  defineProps<{
    policyId?: UUID;
    open?: boolean;
    dialogMaxWidth?: number;
  }>(),
  {
    policyId: undefined,
    open: false,
    dialogMaxWidth: 800,
  },
);

const emit = defineEmits<{
  (event: 'update:open', payload: boolean): void;
}>();

const props = toRefs(input);

const loading = ref(false);
const openModel = computed({
  get: () => props.open.value,
  set: value => emit('update:open', value),
});

const wallet = useWalletStore();

const loadPolicy = async (): Promise<{
  policy: Partial<ProposalPolicy>;
}> => {
  if (props.policyId.value === undefined) {
    const createModel: Partial<ProposalPolicy> = {
      criteria: { AutoAdopted: null },
    };

    return { policy: createModel };
  }

  const result = await wallet.service.getProposalPolicy(props.policyId.value);
  return result;
};
</script>
