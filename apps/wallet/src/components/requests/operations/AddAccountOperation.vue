<template>
  <div v-if="isListMode" class="d-flex flex-column ga-0 text-caption">
    <RequestOperationListRow v-if="accountSetup.configuration.name">
      <template #name>{{ $t('terms.name') }}</template>
      <template #content>
        {{ accountSetup.configuration.name ?? '-' }}
      </template>
    </RequestOperationListRow>
    <RequestOperationListRow v-if="accountSetup.configuration.assets">
      <template #name>{{ $t('terms.assets') }}</template>
      <template #content>
        {{ assetsText }}
      </template>
    </RequestOperationListRow>
  </div>
  <AccountSetupWizard v-else :model-value="accountSetup" mode="view" />
</template>

<script setup lang="ts">
import { Ref, computed, onBeforeMount, ref } from 'vue';
import AccountSetupWizard, {
  AccountSetupWizardModel,
} from '~/components/accounts/wizard/AccountSetupWizard.vue';
import { useDefaultAccountSetupWizardModel } from '~/composables/account.composable';
import { AddAccountOperation, Request } from '~/generated/station/station.did';
import RequestOperationListRow from '../RequestOperationListRow.vue';
import { useStationStore } from '~/stores/station.store';

const props = withDefaults(
  defineProps<{
    request: Request;
    operation: AddAccountOperation;
    mode?: 'list' | 'detail';
  }>(),
  {
    mode: 'list',
  },
);
const station = useStationStore();
const isListMode = computed(() => props.mode === 'list');
const accountSetup: Ref<AccountSetupWizardModel> = ref(useDefaultAccountSetupWizardModel());

const assetsText = computed(() =>
  props.operation.input.assets
    .map(id => station.configuration.details.supported_assets.find(asset => asset.id === id))
    .filter(a => !!a)
    .map(asset => `${asset.name} (${asset.symbol})`)
    .join(', '),
);

onBeforeMount(() => {
  const model: AccountSetupWizardModel = useDefaultAccountSetupWizardModel();
  model.configuration.name = props.operation.input.name;
  model.configuration.assets = props.operation.input.assets;
  model.request_policy.configurationRule = props.operation.input.configs_request_policy?.[0];
  model.request_policy.transferRule = props.operation.input.transfer_request_policy?.[0];
  model.permission.configuration = props.operation.input.configs_permission;
  model.permission.transfer = props.operation.input.transfer_permission;
  model.permission.read = props.operation.input.read_permission;

  accountSetup.value = model;
});
</script>
