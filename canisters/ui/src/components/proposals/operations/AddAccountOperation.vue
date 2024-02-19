<template>
  <div v-if="isListMode" class="d-flex flex-column ga-0 text-caption">
    <ProposalOperationListColumn v-if="formValue.name">
      <template #name>{{ $t('terms.name') }}</template>
      <template #content>
        {{ formValue.name ?? '-' }}
      </template>
    </ProposalOperationListColumn>
    <ProposalOperationListColumn v-if="formValue.blockchain">
      <template #name>{{ $t('terms.blockchain') }}</template>
      <template #content>
        {{ $t(`blockchains.${formValue.blockchain}.name`) }}
      </template>
    </ProposalOperationListColumn>
  </div>
  <AccountForm v-else :model-value="formValue" mode="view" />
</template>

<script setup lang="ts">
import { Ref, computed, onBeforeMount, ref } from 'vue';
import AccountForm from '~/components/accounts/AccountConfigForm.vue';
import { Account, AddAccountOperation, Proposal } from '~/generated/wallet/wallet.did';
import ProposalOperationListColumn from '../ProposalOperationListColumn.vue';

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
const formValue: Ref<Partial<Account>> = ref({});

onBeforeMount(() => {
  const account: Partial<Account> = {};
  account.name = props.operation.input.name;
  account.blockchain = props.operation.input.blockchain;
  account.owners = props.operation.input.owners;
  account.standard = props.operation.input.standard;
  account.policies = props.operation.input.policies;
  account.metadata = props.operation.input.metadata;

  formValue.value = account;
});
</script>
