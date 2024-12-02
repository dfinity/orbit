<template>
  <section class="d-flex flex-column ga-2">
    <CanisterConfigureMethodCallDialog
      :open="canisterConfigureMethodCallDialog.open"
      :canister-id="props.canisterId"
      :method="canisterConfigureMethodCallDialog.method"
      :already-configured-methods="canisterConfigureMethodCallDialog.alreadyConfiguredMethods"
      :canister-candid-idl="props.canisterCandidIdl"
      @update:open="canisterConfigureMethodCallDialog.open = $event"
    />
    <VRow v-if="configuredMethodCalls.length" data-test-id="method-list">
      <VCol v-for="(method, idx) in configuredMethodCalls" :key="idx" cols="12" class="d-flex pb-0">
        <VCard width="100%">
          <VToolbar color="transparent" class="pr-4" density="compact">
            <VToolbarTitle class="text-subtitle-1">
              <VIcon :icon="mdiCodeBraces" size="small" />
              {{ method.methodName }}
            </VToolbarTitle>
            <div v-if="!readonly" class="d-flex flex-nowrap gap-1">
              <ActionBtn
                size="small"
                density="comfortable"
                :icon="mdiTrashCan"
                :submit="
                  _ => {
                    return station.service.editExternalCanisterSettings(props.canisterId, {
                      description: [],
                      labels: [],
                      change_metadata: [],
                      name: [],
                      state: [],
                      permissions: [
                        {
                          change: [],
                          read: [],
                          calls: [
                            {
                              OverrideSpecifiedByExecutionValidationMethodPairs: [
                                {
                                  method_configuration: {
                                    execution_method: method.methodName,
                                    validation_method: method.validationTarget,
                                  },
                                  allow: [],
                                },
                              ],
                            },
                          ],
                        },
                      ],
                      request_policies: [
                        {
                          change: [],
                          calls: [
                            {
                              OverrideSpecifiedByExecutionValidationMethodPairs: [
                                {
                                  method_configuration: {
                                    execution_method: method.methodName,
                                    validation_method: method.validationTarget,
                                  },
                                  policies: [],
                                },
                              ],
                            },
                          ],
                        },
                      ],
                    });
                  }
                "
                @opened="emit('editing', true)"
                @closed="emit('editing', false)"
                @failed="useOnFailedOperation"
                @submitted="
                  request => {
                    if (request && variantIs(request.status, 'Approved')) {
                      removeConfiguredMethodCallsByIdx(idx);
                    }

                    useOnSuccessfulOperation(request);
                  }
                "
              />
              <VSpacer />
              <VBtn
                :icon="mdiPencil"
                size="small"
                density="comfortable"
                @click="openConfigureMethodCallDialog(idx)"
              />
            </div>
          </VToolbar>
          <template v-if="variantIs(method.validationTarget, 'ValidationMethod')">
            <VDivider />
            <VCardText class="text-caption py-2">
              <ValidationMethodExplainer
                :validation-method="method.validationTarget.ValidationMethod.method_name"
                :validation-canister-id="method.validationTarget.ValidationMethod.canister_id"
                :self-canister-id="props.canisterId"
              />
            </VCardText>
          </template>
        </VCard>
      </VCol>
    </VRow>
    <p v-else data-test-id="empty-method-list">
      {{ $t('external_canisters.call_configuration.no_configuration') }}
    </p>
    <footer class="d-flex flex-md-row flex-column ga-2 mt-4">
      <VBtn
        v-if="!readonly"
        size="small"
        variant="outlined"
        block
        @click="openConfigureMethodCallDialog()"
      >
        {{ $t('external_canisters.call_configuration.add_new_method_pair') }}
      </VBtn>
    </footer>
  </section>
</template>
<script setup lang="ts">
import { Principal } from '@dfinity/principal';
import { mdiCodeBraces, mdiPencil, mdiTrashCan } from '@mdi/js';
import { onMounted, Ref, ref, toRefs, watch } from 'vue';
import { VBtn, VDivider, VIcon } from 'vuetify/components';
import {
  useOnFailedOperation,
  useOnSuccessfulOperation,
} from '~/composables/notifications.composable';
import {
  ExternalCanisterPermissions,
  ExternalCanisterRequestPolicies,
} from '~/generated/station/station.did';
import { mapConfiguredMethodCalls } from '~/mappers/external-canister.mapper';
import { useStationStore } from '~/stores/station.store';
import { assertAndReturn, variantIs } from '~/utils/helper.utils';
import ActionBtn from '../buttons/ActionBtn.vue';
import CanisterConfigureMethodCallDialog from './CanisterConfigureMethodCallDialog.vue';
import { CanisterConfiguredMethodCall } from './external-canisters.types';
import ValidationMethodExplainer from './ValidationMethodExplainer.vue';

const props = withDefaults(
  defineProps<{
    canisterId: Principal;
    requestPolicies?: ExternalCanisterRequestPolicies['calls'];
    permissions?: ExternalCanisterPermissions['calls'];
    readonly?: boolean;
    canisterCandidIdl?: string;
  }>(),
  {
    requestPolicies: () => [],
    permissions: () => [],
    readonly: false,
    canisterCandidIdl: undefined,
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

const emit = defineEmits<{
  (event: 'editing', payload: boolean): void;
}>();

const station = useStationStore();
const configuredMethodCalls = ref([]) as Ref<CanisterConfiguredMethodCall[]>;

const openConfigureMethodCallDialog = (idx?: number) => {
  const method =
    idx !== undefined
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

watch(
  () => canisterConfigureMethodCallDialog.value.open,
  isEditing => {
    emit('editing', isEditing);
  },
  { deep: true },
);

const syncConfiguredMethodCalls = () => {
  configuredMethodCalls.value = mapConfiguredMethodCalls({
    requestPolicies: props.requestPolicies,
    permissions: props.permissions,
  });
};

// This is simply to make the UI more responsive in case the request is approved immediately
const removeConfiguredMethodCallsByIdx = (idx: number) => {
  configuredMethodCalls.value.splice(idx, 1);
};

watch(
  () => props.requestPolicies,
  () => syncConfiguredMethodCalls(),
);

watch(
  () => props.permissions,
  () => syncConfiguredMethodCalls(),
);

onMounted(() => syncConfiguredMethodCalls());
</script>
