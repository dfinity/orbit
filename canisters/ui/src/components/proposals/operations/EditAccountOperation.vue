<template>
  <div v-if="isListMode" class="d-flex flex-column ga-0 text-caption">
    <ProposalOperationListColumn v-if="formValue.name">
      <template #name>{{ $t('terms.name') }}</template>
      <template #content>
        {{ formValue.name ?? '-' }}
      </template>
    </ProposalOperationListColumn>
  </div>
  <AccountForm v-else :model-value="formValue" mode="view" />
</template>

<script setup lang="ts">
import { Ref, computed, onBeforeMount, ref } from 'vue';
import AccountForm from '~/components/accounts/AccountConfigForm.vue';
import { Account, EditAccountOperation, Proposal } from '~/generated/wallet/wallet.did';
import ProposalOperationListColumn from '../ProposalOperationListColumn.vue';

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
const formValue: Ref<Partial<Account>> = ref({});

onBeforeMount(() => {
  const account: Partial<Account> = {};
  account.id = props.operation.input.account_id;
  if (props.operation.input.name?.[0]) {
    account.name = props.operation.input.name[0];
  }
  if (props.operation.input.owners?.[0]) {
    account.owners = props.operation.input.owners[0];
  }
  if (props.operation.input.policies?.[0]) {
    account.policies = props.operation.input.policies[0];
  }

  formValue.value = account;
});
</script>
