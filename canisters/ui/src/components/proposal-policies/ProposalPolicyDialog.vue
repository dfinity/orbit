<template>
  <VDialog
    v-model="openModel"
    :persistent="loading || saving"
    transition="dialog-bottom-transition"
    scrollable
    :max-width="props.dialogMaxWidth.value"
  >
    <DataLoader
      v-slot="{ data }"
      :load="loadPolicy"
      @loading="loading = $event"
      @loaded="proposalPolicy = $event.policy"
    >
      <VCard :loading="loading">
        <VToolbar color="background">
          <VToolbarTitle>{{ $t('pages.proposal_policies.dialog_title') }}</VToolbarTitle>
          <VBtn :disabled="loading || saving" :icon="mdiClose" @click="openModel = false" />
        </VToolbar>
        <VCardText>
          <ProposalPolicyForm
            v-if="data"
            v-model="proposalPolicy"
            :mode="props.readonly.value ? 'view' : 'edit'"
            @submit="save"
            @valid="valid = $event"
          />
        </VCardText>
        <VCardActions class="pa-3">
          <VSpacer />
          <VBtn
            v-if="!props.readonly.value"
            color="primary"
            variant="elevated"
            :disabled="!canSave"
            :loading="saving"
            @click="save"
          >
            {{ $t('terms.save') }}
          </VBtn>
        </VCardActions>
      </VCard>
    </DataLoader>
  </VDialog>
</template>
<script lang="ts" setup>
import { mdiClose } from '@mdi/js';
import { computed, ref, toRefs } from 'vue';
import {
  VBtn,
  VCard,
  VCardActions,
  VCardText,
  VDialog,
  VSpacer,
  VToolbar,
  VToolbarTitle,
} from 'vuetify/components';
import DataLoader from '~/components/DataLoader.vue';
import ProposalPolicyForm from '~/components/proposal-policies/ProposalPolicyForm.vue';
import {
  useOnFailedOperation,
  useOnSuccessfulOperation,
} from '~/composables/notifications.composable';
import logger from '~/core/logger.core';
import { ProposalPolicy, UUID } from '~/generated/wallet/wallet.did';
import { useWalletStore } from '~/stores/wallet.store';
import { assertAndReturn } from '~/utils/helper.utils';

const input = withDefaults(
  defineProps<{
    policyId?: UUID;
    open?: boolean;
    dialogMaxWidth?: number;
    readonly?: boolean;
  }>(),
  {
    policyId: undefined,
    open: false,
    dialogMaxWidth: 800,
    readonly: false,
  },
);

const emit = defineEmits<{
  (event: 'update:open', payload: boolean): void;
}>();

const props = toRefs(input);
const valid = ref(true);
const loading = ref(false);
const saving = ref(false);
const proposalPolicy = ref<Partial<ProposalPolicy>>({});
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

const canSave = computed(() => {
  return (
    valid.value &&
    !loading.value &&
    !!proposalPolicy.value?.criteria &&
    !!proposalPolicy.value?.specifier
  );
});

const save = async (): Promise<void> => {
  if (!canSave.value) {
    return;
  }

  try {
    saving.value = true;
    if (proposalPolicy.value.id) {
      const proposal = await wallet.service.editProposalPolicy({
        policy_id: proposalPolicy.value.id,
        specifier: [assertAndReturn(proposalPolicy.value.specifier)],
        criteria: [assertAndReturn(proposalPolicy.value.criteria)],
      });

      useOnSuccessfulOperation(proposal);

      openModel.value = false;
      return;
    }

    const proposal = await wallet.service.addProposalPolicy({
      specifier: assertAndReturn(proposalPolicy.value.specifier),
      criteria: assertAndReturn(proposalPolicy.value.criteria),
    });

    useOnSuccessfulOperation(proposal);

    openModel.value = false;
  } catch (error) {
    logger.error(`Failed to save proposal policy ${error}`);

    useOnFailedOperation();
  } finally {
    saving.value = false;
  }
};
</script>
