<template>
  <div class="d-flex flex-column ga-0 text-caption">
    <ProposalOperationListRow>
      <template #name>{{ $t('terms.target') }}</template>
      <template #content>
        {{ target }}
      </template>
    </ProposalOperationListRow>
    <ProposalOperationListRow>
      <template #name>{{ $t('terms.wasm') }}</template>
      <template #content>
        {{ checksum }}
      </template>
    </ProposalOperationListRow>
    <ProposalOperationListRow v-if="!isListMode && argChecksum">
      <template #name>{{ $t('terms.arg') }}</template>
      <template #content>
        {{ argChecksum }}
      </template>
    </ProposalOperationListRow>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { ChangeCanisterOperation, Proposal } from '~/generated/wallet/wallet.did';
import { variantIs } from '~/utils/helper.utils';
import ProposalOperationListRow from '../ProposalOperationListRow.vue';

const props = withDefaults(
  defineProps<{
    proposal: Proposal;
    operation: ChangeCanisterOperation;
    mode?: 'list' | 'detail';
  }>(),
  {
    mode: 'list',
  },
);

const i18n = useI18n();
const isListMode = computed(() => props.mode === 'list');

const target = computed(() => {
  if (variantIs(props.operation.target, 'UpgradeWallet')) {
    return i18n.t('terms.wallet');
  }

  if (variantIs(props.operation.target, 'UpgradeUpgrader')) {
    return i18n.t('terms.upgrader');
  }

  return props.operation.target.UpgradeCanister.toText();
});

const checksum = computed(() => {
  return Buffer.from(props.operation.checksum).toString('hex');
});

const argChecksum = computed(() => {
  if (!props.operation.arg_checksum?.[0]) {
    return undefined;
  }

  return Buffer.from(props.operation.arg_checksum[0]).toString('hex');
});
</script>
