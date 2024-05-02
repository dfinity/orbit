<template>
  <VDialog
    v-model="open"
    :persistent="!canClose"
    transition="dialog-bottom-transition"
    scrollable
    :max-width="props.dialogMaxWidth"
  >
    <DataLoader :load="load" @loading="loading = $event" @loaded="wizard = $event">
      <template #error="{ errorMsg, errorDetails }">
        <ErrorCard
          :title="$t('app.account_dialog_view_title')"
          :error="errorMsg"
          :error-details="errorDetails"
        />
      </template>
      <VCard>
        <VToolbar color="background">
          <VToolbarTitle>
            {{
              props.accountId
                ? $t('app.account_dialog_view_title')
                : $t('app.account_dialog_create_new_title')
            }}
          </VToolbarTitle>
          <VBtn :disabled="!canClose" :icon="mdiClose" @click="open = false" />
        </VToolbar>
        <VDivider />

        <AccountSetupWizard
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
import { mdiClose } from '@mdi/js';
import { computed, ref } from 'vue';
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
import AccountSetupWizard, {
  AccountSetupWizardModel,
} from '~/components/accounts/wizard/AccountSetupWizard.vue';
import ErrorCard from '~/components/ui/ErrorCard.vue';
import {
  useDefaultAccountSetupWizardModel,
  useLoadAccountSetupWizardModel,
} from '~/composables/account.composable';
import {
  useOnFailedOperation,
  useOnSuccessfulOperation,
} from '~/composables/notifications.composable';
import logger from '~/core/logger.core';
import {
  AddAccountOperationInput,
  EditAccountOperationInput,
  Request,
  UUID,
} from '~/generated/station/station.did';
import { useStationStore } from '~/stores/station.store';
import { assertAndReturn } from '~/utils/helper.utils';

const props = withDefaults(
  defineProps<{
    accountId?: UUID;
    open?: boolean;
    dialogMaxWidth?: number;
    readonly?: boolean;
  }>(),
  {
    accountId: undefined,
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
const wizard = ref<AccountSetupWizardModel>(useDefaultAccountSetupWizardModel());
const canClose = computed(() => !loading.value && !submitting.value);
const station = useStationStore();
const open = computed({
  get: () => props.open,
  set: value => emit('update:open', value),
});

const load = async (): Promise<AccountSetupWizardModel> => {
  if (props.accountId === undefined) {
    return useDefaultAccountSetupWizardModel({
      prefilledUserIds: [station.user.id],
    });
  }

  return useLoadAccountSetupWizardModel(props.accountId);
};

const save = async (): Promise<void> => {
  try {
    submitting.value = true;

    const request = props.accountId
      ? await saveChangesToExistingAccount(props.accountId)
      : await createNewAccount();

    useOnSuccessfulOperation(request);

    open.value = false;
  } catch (error) {
    logger.error(`Failed to submit account ${error}`);

    useOnFailedOperation();
  } finally {
    submitting.value = false;
  }
};

const saveChangesToExistingAccount = async (accountId: UUID): Promise<Request> => {
  const changes: Partial<EditAccountOperationInput> = {};
  changes.account_id = accountId;
  changes.name = [assertAndReturn(wizard.value.configuration.name, 'name')];
  changes.configs_request_policy = !wizard.value.request_policy.configurationRule
    ? [{ Remove: null }]
    : [{ Set: assertAndReturn(wizard.value.request_policy.configurationRule) }];
  changes.transfer_request_policy = !wizard.value.request_policy.transferRule
    ? [{ Remove: null }]
    : [{ Set: assertAndReturn(wizard.value.request_policy.transferRule) }];
  changes.read_permission = [assertAndReturn(wizard.value.permission.read, 'read_access')];
  changes.transfer_permission = [
    assertAndReturn(wizard.value.permission.transfer, 'transfer_access'),
  ];
  changes.configs_permission = [
    assertAndReturn(wizard.value.permission.configuration, 'update_access'),
  ];

  return station.service.editAccount(changes as EditAccountOperationInput);
};

const createNewAccount = async (): Promise<Request> => {
  const changes: Partial<AddAccountOperationInput> = {};
  changes.name = assertAndReturn(wizard.value.configuration.name, 'name');
  changes.blockchain = assertAndReturn(wizard.value.configuration.blockchain, 'blockchain');
  changes.standard = assertAndReturn(wizard.value.configuration.standard, 'standard');
  changes.configs_request_policy = wizard.value.request_policy.configurationRule
    ? [wizard.value.request_policy.configurationRule]
    : [];
  changes.transfer_request_policy = wizard.value.request_policy.transferRule
    ? [wizard.value.request_policy.transferRule]
    : [];
  changes.read_permission = assertAndReturn(wizard.value.permission.read, 'read_access');
  changes.transfer_permission = assertAndReturn(
    wizard.value.permission.transfer,
    'transfer_access',
  );
  changes.configs_permission = assertAndReturn(
    wizard.value.permission.configuration,
    'update_access',
  );
  changes.metadata = [];

  return station.service.addAccount(changes as AddAccountOperationInput);
};
</script>
