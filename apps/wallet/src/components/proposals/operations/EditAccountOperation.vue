<template>
  <div v-if="isListMode" class="d-flex flex-column ga-0 text-caption">
    <ProposalOperationListRow v-if="props.operation.input.name?.[0]">
      <template #name>{{ $t('terms.name') }}</template>
      <template #content>
        {{ props.operation.input.name[0] ?? '-' }}
      </template>
    </ProposalOperationListRow>
  </div>
  <LoadingMessage v-else-if="loading" />
  <AccountSetupWizard v-else :model-value="model" mode="view" />
</template>

<script setup lang="ts">
import { Ref, computed, onBeforeMount, ref } from 'vue';
import AccountSetupWizard, {
  AccountSetupWizardModel,
} from '~/components/accounts/wizard/AccountSetupWizard.vue';
import LoadingMessage from '~/components/LoadingMessage.vue';
import {
  useDefaultAccountSetupWizardModel,
  useLoadAccountSetupWizardModel,
} from '~/composables/account.composable';
import logger from '~/core/logger.core';
import { EditAccountOperation, Proposal } from '~/generated/wallet/wallet.did';
import { variantIs } from '~/utils/helper.utils';
import ProposalOperationListRow from '../ProposalOperationListRow.vue';

const props = withDefaults(
  defineProps<{
    proposal: Proposal;
    operation: EditAccountOperation;
    mode?: 'list' | 'detail';
  }>(),
  {
    mode: 'list',
  },
);

const isListMode = computed(() => props.mode === 'list');
const model: Ref<AccountSetupWizardModel> = ref(useDefaultAccountSetupWizardModel());
const loading = ref(false);

const fetchDetails = async () => {
  try {
    if (loading.value || isListMode.value) {
      return;
    }

    loading.value = true;
    model.value = await useLoadAccountSetupWizardModel(props.operation.input.account_id);

    if (props.operation.input.name?.[0]) {
      model.value.configuration.name = props.operation.input.name[0];
    }

    if (props.operation.input.update_approval_policy?.[0]) {
      model.value.approval_policy.configurationCriteria = variantIs(
        props.operation.input.update_approval_policy[0],
        'Remove',
      )
        ? undefined
        : props.operation.input.update_approval_policy[0].Set;
    }

    if (props.operation.input.transfer_approval_policy?.[0]) {
      model.value.approval_policy.transferCriteria = variantIs(
        props.operation.input.transfer_approval_policy[0],
        'Remove',
      )
        ? undefined
        : props.operation.input.transfer_approval_policy[0].Set;
    }

    if (props.operation.input.read_access_policy?.[0]) {
      model.value.access_policy.read = props.operation.input.read_access_policy?.[0];
    }

    if (props.operation.input.transfer_access_policy?.[0]) {
      model.value.access_policy.transfer = props.operation.input.transfer_access_policy?.[0];
    }

    if (props.operation.input.update_access_policy?.[0]) {
      model.value.access_policy.configuration = props.operation.input.update_access_policy?.[0];
    }
  } catch (e) {
    logger.error('Failed to fetch account details', e);
  } finally {
    loading.value = false;
  }
};

onBeforeMount(() => {
  const model: AccountSetupWizardModel = useDefaultAccountSetupWizardModel();
  model.configuration.id = props.operation.input.account_id;

  fetchDetails();
});
</script>
