<template>
  <div v-if="isListMode" class="d-flex flex-column ga-0 text-caption">
    <ProposalOperationListRow v-if="formValue.name">
      <template #name>{{ $t('terms.name') }}</template>
      <template #content>
        {{ formValue.name ?? '-' }}
      </template>
    </ProposalOperationListRow>
  </div>
  <AccountForm v-else :model-value="formValue" mode="view" />
</template>

<script setup lang="ts">
import { Ref, computed, onBeforeMount, ref } from 'vue';
import AccountForm from '~/components/accounts/AccountConfigForm.vue';
import { Account, EditAccountOperation, Proposal } from '~/generated/wallet/wallet.did';
import ProposalOperationListRow from '../ProposalOperationListRow.vue';
import { variantIs } from '~/utils/helper.utils';

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

  if (props.operation.input.transfer_approval_policy?.[0]) {
    if (variantIs(props.operation.input.transfer_approval_policy[0], 'Set')) {
      account.transfer_approval_policy = [props.operation.input.transfer_approval_policy[0].Set];
    } else if (variantIs(props.operation.input.transfer_approval_policy[0], 'Remove')) {
      account.transfer_approval_policy = [];
    }
  }

  if (props.operation.input.update_approval_policy?.[0]) {
    if (variantIs(props.operation.input.update_approval_policy[0], 'Set')) {
      account.update_approval_policy = [props.operation.input.update_approval_policy[0].Set];
    } else if (variantIs(props.operation.input.update_approval_policy[0], 'Remove')) {
      account.update_approval_policy = [];
    }
  }

  formValue.value = account;
});
</script>
