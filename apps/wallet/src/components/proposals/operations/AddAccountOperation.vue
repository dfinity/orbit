<template>
  <div v-if="isListMode" class="d-flex flex-column ga-0 text-caption">
    <ProposalOperationListRow v-if="accountSetup.configuration.name">
      <template #name>{{ $t('terms.name') }}</template>
      <template #content>
        {{ accountSetup.configuration.name ?? '-' }}
      </template>
    </ProposalOperationListRow>
    <ProposalOperationListRow v-if="accountSetup.configuration.blockchain">
      <template #name>{{ $t('terms.blockchain') }}</template>
      <template #content>
        {{ $t(`blockchains.${accountSetup.configuration.blockchain}.name`) }}
      </template>
    </ProposalOperationListRow>
  </div>
  <AccountSetupWizard v-else :model-value="accountSetup" mode="view" />
</template>

<script setup lang="ts">
import { Ref, computed, onBeforeMount, ref } from 'vue';
import AccountSetupWizard, {
  AccountSetupWizardModel,
} from '~/components/accounts/wizard/AccountSetupWizard.vue';
import { useDefaultAccountSetupWizardModel } from '~/composables/account.composable';
import { AddAccountOperation, Proposal } from '~/generated/station/station.did';
import ProposalOperationListRow from '../ProposalOperationListRow.vue';

const props = withDefaults(
  defineProps<{
    proposal: Proposal;
    operation: AddAccountOperation;
    mode?: 'list' | 'detail';
  }>(),
  {
    mode: 'list',
  },
);

const isListMode = computed(() => props.mode === 'list');
const accountSetup: Ref<AccountSetupWizardModel> = ref(useDefaultAccountSetupWizardModel());

onBeforeMount(() => {
  const model: AccountSetupWizardModel = useDefaultAccountSetupWizardModel();
  model.configuration.name = props.operation.input.name;
  model.configuration.blockchain = props.operation.input.blockchain;
  model.configuration.standard = props.operation.input.standard;
  model.approval_policy.configurationCriteria = props.operation.input.update_approval_policy?.[0];
  model.approval_policy.transferCriteria = props.operation.input.transfer_approval_policy?.[0];
  model.permission.configuration = props.operation.input.update_permission;
  model.permission.transfer = props.operation.input.transfer_permission;
  model.permission.read = props.operation.input.read_permission;

  accountSetup.value = model;
});
</script>
