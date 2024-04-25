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
  Proposal,
  UUID,
} from '~/generated/station/station.did';
import { useWalletStore } from '~/stores/wallet.store';
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
const wallet = useWalletStore();
const open = computed({
  get: () => props.open,
  set: value => emit('update:open', value),
});

const load = async (): Promise<AccountSetupWizardModel> => {
  if (props.accountId === undefined) {
    return useDefaultAccountSetupWizardModel({
      prefilledUserIds: [wallet.user.id],
    });
  }

  return useLoadAccountSetupWizardModel(props.accountId);
};

const save = async (): Promise<void> => {
  try {
    submitting.value = true;

    const proposal = props.accountId
      ? await saveChangesToExistingAccount(props.accountId)
      : await createNewAccount();

    useOnSuccessfulOperation(proposal);

    open.value = false;
  } catch (error) {
    logger.error(`Failed to submit account ${error}`);

    useOnFailedOperation();
  } finally {
    submitting.value = false;
  }
};

const saveChangesToExistingAccount = async (accountId: UUID): Promise<Proposal> => {
  const changes: Partial<EditAccountOperationInput> = {};
  changes.account_id = accountId;
  changes.name = [assertAndReturn(wizard.value.configuration.name, 'name')];
  changes.update_approval_policy = !wizard.value.approval_policy.configurationCriteria
    ? [{ Remove: null }]
    : [{ Set: assertAndReturn(wizard.value.approval_policy.configurationCriteria) }];
  changes.transfer_approval_policy = !wizard.value.approval_policy.transferCriteria
    ? [{ Remove: null }]
    : [{ Set: assertAndReturn(wizard.value.approval_policy.transferCriteria) }];
  changes.read_access_policy = [assertAndReturn(wizard.value.access_policy.read, 'read_access')];
  changes.transfer_access_policy = [
    assertAndReturn(wizard.value.access_policy.transfer, 'transfer_access'),
  ];
  changes.update_access_policy = [
    assertAndReturn(wizard.value.access_policy.configuration, 'update_access'),
  ];

  return wallet.service.editAccount(changes as EditAccountOperationInput);
};

const createNewAccount = async (): Promise<Proposal> => {
  const changes: Partial<AddAccountOperationInput> = {};
  changes.name = assertAndReturn(wizard.value.configuration.name, 'name');
  changes.blockchain = assertAndReturn(wizard.value.configuration.blockchain, 'blockchain');
  changes.standard = assertAndReturn(wizard.value.configuration.standard, 'standard');
  changes.update_approval_policy = wizard.value.approval_policy.configurationCriteria
    ? [wizard.value.approval_policy.configurationCriteria]
    : [];
  changes.transfer_approval_policy = wizard.value.approval_policy.transferCriteria
    ? [wizard.value.approval_policy.transferCriteria]
    : [];
  changes.read_access_policy = assertAndReturn(wizard.value.access_policy.read, 'read_access');
  changes.transfer_access_policy = assertAndReturn(
    wizard.value.access_policy.transfer,
    'transfer_access',
  );
  changes.update_access_policy = assertAndReturn(
    wizard.value.access_policy.configuration,
    'update_access',
  );
  changes.metadata = [];

  return wallet.service.addAccount(changes as AddAccountOperationInput);
};
</script>
