<template>
  <div class="d-flex flex-column ga-0 text-caption">
    <RequestOperationListRow>
      <template #name>{{ $t('terms.target') }}</template>
      <template #content>
        {{ target }}
      </template>
    </RequestOperationListRow>
    <RequestOperationListRow>
      <template #name>{{ $t('terms.wasm') }}</template>
      <template #content>
        {{ props.operation.module_checksum }}
      </template>
    </RequestOperationListRow>
    <RequestOperationListRow v-if="props.operation.arg_checksum?.[0]">
      <template #name>{{ $t('terms.arg') }}</template>
      <template #content>
        {{ props.operation.arg_checksum[0] }}
      </template>
    </RequestOperationListRow>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { ChangeCanisterOperation, Request } from '~/generated/station/station.did';
import { variantIs } from '~/utils/helper.utils';
import RequestOperationListRow from '../RequestOperationListRow.vue';

const props = withDefaults(
  defineProps<{
    request: Request;
    operation: ChangeCanisterOperation;
    mode?: 'list' | 'detail';
  }>(),
  {
    mode: 'list',
  },
);

const i18n = useI18n();

const target = computed(() => {
  if (variantIs(props.operation.target, 'UpgradeStation')) {
    return i18n.t('terms.station');
  }

  if (variantIs(props.operation.target, 'UpgradeUpgrader')) {
    return i18n.t('terms.upgrader');
  }

  if (variantIs(props.operation.target, 'UpgradeCanister')) {
    return props.operation.target.UpgradeCanister.toText();
  }

  return props.operation.target.InstallCanister.canister_id.toText();
});
</script>
