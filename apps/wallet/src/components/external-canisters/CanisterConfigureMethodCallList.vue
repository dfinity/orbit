<template>
  <section class="d-flex flex-column ga-2">
    <CanisterConfigureMethodCallDialog
      :open="canisterConfigureMethodCallDialog.open"
      :canister-id="props.canisterId"
      :method="canisterConfigureMethodCallDialog.method"
      :already-configured-methods="canisterConfigureMethodCallDialog.alreadyConfiguredMethods"
      @update:open="canisterConfigureMethodCallDialog.open = $event"
    />
    <header class="d-flex flex-md-row flex-column ga-2">
      <div class="text-h6 text-weight-bold d-flex align-center ga-1 flex-grow-1">
        <VIcon :icon="mdiDatabaseArrowLeftOutline" size="x-small" />
        {{ $t('external_canisters.call_configuration.title') }}
      </div>
      <VBtn
        v-if="!readonly"
        size="small"
        variant="outlined"
        @click="openConfigureMethodCallDialog()"
      >
        {{ $t('external_canisters.call_configuration.add_new') }}
      </VBtn>
    </header>
    <VDivider />
    <div v-if="configuredMethodCalls.length" class="d-flex">
      <div v-for="(method, idx) in configuredMethodCalls" :key="idx">
        {{ method }}
        <VBtn
          v-if="!readonly"
          size="small"
          variant="outlined"
          @click="openConfigureMethodCallDialog(idx)"
        >
          {{ $t('external_canisters.call_configuration.edit') }}
        </VBtn>
      </div>
    </div>
    <p v-else>
      {{ $t('external_canisters.call_configuration.no_configuration') }}
    </p>
  </section>
</template>
<script setup lang="ts">
import { Principal } from '@dfinity/principal';
import { mdiDatabaseArrowLeftOutline } from '@mdi/js';
import { onMounted, Ref, ref, toRefs } from 'vue';
import { VBtn, VDivider, VIcon } from 'vuetify/components';
import {
  ExternalCanisterPermissions,
  ExternalCanisterRequestPolicies,
  ValidationMethodResourceTarget,
} from '~/generated/station/station.did';
import { mapMethodCallConfigurationToKey } from '~/mappers/external-canister.mapper';
import { assertAndReturn } from '~/utils/helper.utils';
import CanisterConfigureMethodCallDialog from './CanisterConfigureMethodCallDialog.vue';
import { CanisterConfiguredMethodCall } from './external-canisters.types';

const props = withDefaults(
  defineProps<{
    canisterId: Principal;
    requestPolicies?: ExternalCanisterRequestPolicies['calls'];
    permissions?: ExternalCanisterPermissions['calls'];
    readonly?: boolean;
  }>(),
  {
    requestPolicies: () => [],
    permissions: () => [],
    readonly: false,
  },
);

const { readonly } = toRefs(props);

const canisterConfigureMethodCallDialog = ref({
  open: false,
  method: {
    methodName: '',
    requestPolicies: [],
    permission: undefined,
    validationTarget: { No: null },
  },
  alreadyConfiguredMethods: [],
}) as Ref<{
  open: boolean;
  method: CanisterConfiguredMethodCall;
  alreadyConfiguredMethods: CanisterConfiguredMethodCall[];
}>;

const configuredMethodCalls = ref([]) as Ref<CanisterConfiguredMethodCall[]>;

const openConfigureMethodCallDialog = (idx?: number) => {
  const method = idx
    ? assertAndReturn(configuredMethodCalls.value[idx], 'configured method should exist')
    : {
        methodName: '',
        requestPolicies: [],
        permission: undefined,
        validationTarget: { No: null },
      };

  canisterConfigureMethodCallDialog.value = {
    method,
    alreadyConfiguredMethods: configuredMethodCalls.value,
    open: true,
  };
};

const syncConfiguredMethodCalls = () => {
  const updatedConfiguredMethodCalls: Map<string, CanisterConfiguredMethodCall> = new Map();

  const getOrDefault = (
    methodName: string,
    validationTarget: ValidationMethodResourceTarget,
  ): CanisterConfiguredMethodCall =>
    updatedConfiguredMethodCalls.get(
      mapMethodCallConfigurationToKey({
        executionMethod: methodName,
        validationMethod: validationTarget,
      }),
    ) ?? {
      methodName,
      requestPolicies: [],
      permission: undefined,
      validationTarget,
    };

  for (const policy of props.requestPolicies) {
    const methodCallEntry = getOrDefault(policy.execution_method, policy.validation_method);

    methodCallEntry.requestPolicies.push({
      rule: policy.rule,
      policy_id: [policy.policy_id],
    });

    updatedConfiguredMethodCalls.set(
      mapMethodCallConfigurationToKey({
        executionMethod: policy.execution_method,
        validationMethod: policy.validation_method,
      }),
      methodCallEntry,
    );
  }

  for (const permission of props.permissions) {
    const methodCallEntry = getOrDefault(permission.execution_method, permission.validation_method);

    methodCallEntry.permission = permission.allow;

    updatedConfiguredMethodCalls.set(
      mapMethodCallConfigurationToKey({
        executionMethod: permission.execution_method,
        validationMethod: permission.validation_method,
      }),
      methodCallEntry,
    );
  }

  configuredMethodCalls.value = Array.from(updatedConfiguredMethodCalls.values());
};

onMounted(() => {
  syncConfiguredMethodCalls();
});
</script>
