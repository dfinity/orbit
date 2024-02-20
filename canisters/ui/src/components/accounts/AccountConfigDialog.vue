<template>
  <VDialog
    v-model="openModel"
    :persistent="loading || saving"
    transition="dialog-bottom-transition"
    scrollable
    :max-width="props.dialogMaxWidth.value"
  >
    <DataLoader
      v-slot="{ data }"
      :load="loadAccount"
      @loading="loading = $event"
      @loaded="account = $event.account"
    >
      <VCard :loading="loading">
        <VToolbar dark color="surface">
          <VToolbarTitle>{{ $t('terms.account') }}</VToolbarTitle>
          <VBtn :disabled="loading || saving" :icon="mdiClose" dark @click="openModel = false" />
        </VToolbar>
        <VCardText>
          <AccountConfigForm
            v-if="data"
            v-model="account"
            v-model:trigger-submit="triggerSubmit"
            :display="{
              id: true,
              asset: account.id ? false : true,
            }"
            :mode="props.readonly.value ? 'view' : 'edit'"
            @submit="save"
            @valid="valid = $event"
          />
        </VCardText>
        <VCardActions class="pa-3">
          <VSpacer />
          <VBtn
            v-if="!props.readonly.value"
            :disabled="!canSave"
            :loading="saving"
            @click="triggerSubmit = true"
          >
            {{ props.accountId.value ? $t('terms.save') : $t('terms.create') }}
          </VBtn>
        </VCardActions>
      </VCard>
    </DataLoader>
  </VDialog>
</template>
<script lang="ts" setup>
import { mdiClose } from '@mdi/js';
import { computed, ref, toRefs } from 'vue';
import DataLoader from '~/components/DataLoader.vue';
import AccountConfigForm from '~/components/accounts/AccountConfigForm.vue';
import {
  useOnFailedOperation,
  useOnSuccessfulOperation,
} from '~/composables/notifications.composable';
import logger from '~/core/logger.core';
import { Account, UUID } from '~/generated/wallet/wallet.did';
import { useWalletStore } from '~/stores/wallet.store';
import { BlockchainStandard, BlockchainType } from '~/types/chain.types';
import { assertAndReturn } from '~/utils/helper.utils';

const input = withDefaults(
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

const props = toRefs(input);
const valid = ref(true);
const loading = ref(false);
const saving = ref(false);
const account = ref<Partial<Account>>({});
const openModel = computed({
  get: () => props.open.value,
  set: value => emit('update:open', value),
});

const wallet = useWalletStore();

const loadAccount = async (): Promise<{
  account: Partial<Account>;
}> => {
  if (props.accountId.value === undefined) {
    const createModel: Partial<Account> = {
      blockchain: BlockchainType.InternetComputer,
      standard: BlockchainStandard.Native,
      policies: {
        edit: [{ ApprovalThreshold: { threshold: 100, voters: { Owner: null } } }],
        transfer: [{ ApprovalThreshold: { threshold: 100, voters: { Owner: null } } }],
      },
    };

    return { account: createModel };
  }

  const result = await wallet.service.getAccount({ account_id: props.accountId.value });
  return { account: result.account };
};

const canSave = computed(() => {
  return (
    valid.value &&
    !loading.value &&
    !!account.value.policies &&
    !!account.value.policies.edit.length &&
    !!account.value.policies.transfer.length
  );
});

const triggerSubmit = ref(false);

const save = async (): Promise<void> => {
  if (!canSave.value) {
    return;
  }

  try {
    saving.value = true;
    if (account.value.id) {
      const proposal = await wallet.service.editAccount({
        account_id: account.value.id,
        name: [assertAndReturn(account.value.name)],
        owners: [assertAndReturn(account.value.owners)],
        policies: [assertAndReturn(account.value.policies)],
      });

      useOnSuccessfulOperation(proposal);

      openModel.value = false;
      return;
    }

    const proposal = await wallet.service.addAccount({
      name: assertAndReturn(account.value.name, 'name'),
      owners: assertAndReturn(account.value.owners, 'owners'),
      blockchain: assertAndReturn(account.value.blockchain, 'blockchain'),
      standard: assertAndReturn(account.value.standard, 'standard'),
      metadata: assertAndReturn(account.value.metadata, 'metadata'),
      policies: assertAndReturn(account.value.policies, 'policies'),
    });

    useOnSuccessfulOperation(proposal);

    openModel.value = false;
  } catch (error) {
    logger.error(`Failed to save account config ${error}`);

    useOnFailedOperation();
  } finally {
    saving.value = false;
  }
};
</script>
