<template>
  <VDialog
    v-bind="$attrs"
    v-model="open"
    :persistent="!canClose"
    transition="dialog-bottom-transition"
    scrollable
    :max-width="props.dialogMaxWidth"
  >
    <DataLoader :load="load" @loading="loading = $event" @loaded="wizard = $event">
      <template #error="{ errorMsg, errorDetails }">
        <ErrorCard
          data-test-id="canister-setup-error-card"
          :title="dialogTitle"
          :error="errorMsg"
          :error-details="errorDetails"
        />
      </template>
      <VCard data-test-id="canister-setup-ok-card">
        <VToolbar color="background">
          <VToolbarTitle>
            {{ dialogTitle }}
          </VToolbarTitle>
          <VBtn :disabled="!canClose" :icon="mdiClose" @click="open = false" />
        </VToolbar>
        <VDivider />

        <CanisterSetupWizard
          v-if="!loading"
          v-model="wizard"
          :mode="props.readonly ? 'view' : 'edit'"
          :saving="submitting"
          @submit="save"
        />

        <VCardText v-if="loading" class="py-8">
          <LoadingMessage />
        </VCardText>
      </VCard>
    </DataLoader>
  </VDialog>
</template>
<script lang="ts" setup>
import { Principal } from '@dfinity/principal';
import { mdiClose } from '@mdi/js';
import { Ref, computed, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import {
  VBtn,
  VCard,
  VCardText,
  VDialog,
  VDivider,
  VToolbar,
  VToolbarTitle,
} from 'vuetify/components';
import DataLoader from '~/components/DataLoader.vue';
import LoadingMessage from '~/components/LoadingMessage.vue';
import { CanisterWizardModel } from '~/components/external-canisters/wizard/wizard.types';
import ErrorCard from '~/components/ui/ErrorCard.vue';
import {
  useDefaultExternalCanisterSetupWizardModel,
  useLoadExternaLCanisterSetupWizardModel,
} from '~/composables/external-canisters.composable';
import {
  useOnFailedOperation,
  useOnSuccessfulOperation,
} from '~/composables/notifications.composable';
import logger from '~/core/logger.core';
import {
  ConfigureExternalCanisterSettingsInput,
  CreateExternalCanisterOperationInput,
  Request,
  RequestPolicyRule,
} from '~/generated/station/station.did';
import { mapExternalCanisterStateEnumToVariant } from '~/mappers/external-canister.mapper';
import { useStationStore } from '~/stores/station.store';
import { assertAndReturn } from '~/utils/helper.utils';
import CanisterSetupWizard from './wizard/CanisterSetupWizard.vue';

const props = withDefaults(
  defineProps<{
    canisterId?: Principal;
    open?: boolean;
    dialogMaxWidth?: number;
    readonly?: boolean;
  }>(),
  {
    canisterId: undefined,
    open: false,
    dialogMaxWidth: 800,
    readonly: false,
  },
);

const emit = defineEmits<{
  (event: 'update:open', payload: boolean): void;
}>();

const loading = ref(false);
const submitting = ref(false);
const i18n = useI18n();
const station = useStationStore();
const wizard = ref<CanisterWizardModel>(
  useDefaultExternalCanisterSetupWizardModel(),
) as Ref<CanisterWizardModel>;
const canClose = computed(() => !loading.value && !submitting.value);
const open = computed({
  get: () => props.open,
  set: value => emit('update:open', value),
});
const dialogTitle = computed(() =>
  props.canisterId
    ? i18n.t('pages.external_canisters.edit_canister_title')
    : i18n.t('pages.external_canisters.add_new_canister_title'),
);

const load = async () => {
  if (!props.canisterId) {
    return useDefaultExternalCanisterSetupWizardModel({
      prefilledUserIds: [station.user.id],
    });
  }

  return useLoadExternaLCanisterSetupWizardModel(props.canisterId);
};

const save = async (): Promise<void> => {
  try {
    submitting.value = true;

    const request = props.canisterId
      ? await saveChangesToExistingExternalCanister(props.canisterId)
      : await createNewExternalCanister();

    useOnSuccessfulOperation(request);

    open.value = false;
  } catch (error) {
    logger.error(`Failed to submit external canister change request: ${error}`);

    useOnFailedOperation();
  } finally {
    submitting.value = false;
  }
};

const saveChangesToExistingExternalCanister = async (canisterId: Principal): Promise<Request> => {
  const settings: Partial<ConfigureExternalCanisterSettingsInput> = {};
  settings.name = [assertAndReturn(wizard.value.configuration.name, 'name')];
  settings.labels = wizard.value.configuration.labels ? [wizard.value.configuration.labels] : [];
  settings.state = wizard.value.configuration.state
    ? [mapExternalCanisterStateEnumToVariant(wizard.value.configuration.state)]
    : [];
  settings.description = wizard.value.configuration.description?.length
    ? [wizard.value.configuration.description]
    : [''];
  settings.permissions = [
    {
      read: [assertAndReturn(wizard.value.permission.read, 'read permission')],
      change: [assertAndReturn(wizard.value.permission.change, 'change permission')],
      calls: [], // optional field, not updating calls through this dialog
    },
  ];
  settings.change_metadata = [];
  settings.request_policies = [
    {
      calls: [], // optional field, not updating calls through this dialog
      change: [
        wizard.value.approvalPolicy.change
          .filter(item => item.rule !== undefined)
          .map(item => ({
            policy_id: item.policy_id ? [item.policy_id] : [],
            rule: item.rule as RequestPolicyRule,
          })),
      ],
    },
  ];

  return station.service.editExternalCanisterSettings(
    canisterId,
    settings as ConfigureExternalCanisterSettingsInput,
  );
};

const createNewExternalCanister = async (): Promise<Request> => {
  const changes: Partial<CreateExternalCanisterOperationInput> = {};
  changes.name = assertAndReturn(wizard.value.configuration.name, 'name');
  changes.description = wizard.value.configuration.description?.length
    ? [wizard.value.configuration.description]
    : [];
  changes.labels = wizard.value.configuration.labels ? [wizard.value.configuration.labels] : [];
  changes.permissions = {
    read: assertAndReturn(wizard.value.permission.read, 'read permission'),
    change: assertAndReturn(wizard.value.permission.change, 'change permission'),
    calls: [],
  };
  changes.request_policies = {
    calls: [],
    change: wizard.value.approvalPolicy.change
      .filter(item => item.rule !== undefined)
      .map(item => ({
        policy_id: [],
        rule: item.rule as RequestPolicyRule,
      })),
  };
  changes.metadata = [];
  if (wizard.value.configuration.canisterId) {
    changes.kind = {
      AddExisting: {
        canister_id: wizard.value.configuration.canisterId as Principal,
      },
    };
  } else {
    changes.kind = {
      CreateNew: {
        initial_cycles: wizard.value.configuration.maybe_with_initial_cycles
          ? [wizard.value.configuration.maybe_with_initial_cycles]
          : [],
        // TODO: implement subnet selection
        subnet_selection: [],
      },
    };
  }

  return station.service.addExternalCanister(changes as CreateExternalCanisterOperationInput);
};
</script>
