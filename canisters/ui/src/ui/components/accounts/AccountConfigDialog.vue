<template>
  <VDialog v-model="show" persistent transition="dialog-bottom-transition" scrollable>
    <VCard :loading="loading">
      <VToolbar dark color="primary">
        <VToolbarTitle v-if="props.mode === 'add'">{{ $t('terms.new_account') }}</VToolbarTitle>
        <VToolbarTitle v-else-if="props.mode === 'edit'">{{
          $t('terms.edit_account')
        }}</VToolbarTitle>
        <VToolbarTitle v-else>{{ $t('terms.account') }}</VToolbarTitle>
        <VBtn :icon="mdiClose" variant="text" dark @click="closeDialog" />
      </VToolbar>
      <VCardText>
        <AccountForm
          v-model="submitted"
          :mode="props.mode"
          :account="props.account"
          @loading="isLoading => (loading = isLoading)"
          @updated="isChanged => (hasChanges = isChanged)"
          @saved="onSave"
        />
      </VCardText>
      <VCardActions v-if="props.mode !== 'view'">
        <VSpacer />
        <VBtn
          :disabled="!hasChanges"
          :loading="loading"
          color="primary"
          variant="flat"
          type="submit"
          @click="submit"
        >
          {{ props.mode === 'add' ? $t('forms.create') : $t('forms.edit') }}
        </VBtn>
      </VCardActions>
    </VCard>
  </VDialog>
</template>
<script lang="ts" setup>
import { computed, ref } from 'vue';
import AccountForm from './AccountForm.vue';
import { mdiClose } from '@mdi/js';
import { Account } from '~/generated/wallet/wallet.did';
import { useAppStore } from '~/ui/stores';
import { i18n } from '~/ui/modules';

const props = withDefaults(
  defineProps<{
    modelValue: boolean;
    mode: 'add' | 'edit' | 'view';
    account?: Account;
  }>(),
  {
    modelValue: false,
    mode: 'add',
    account: undefined,
  },
);

const emit = defineEmits<{
  (event: 'update:modelValue', value: boolean): void;
  (event: 'saved'): void;
}>();

const show = computed({
  get: () => props.modelValue,
  set: (value: boolean) => {
    if (!value) {
      emit('update:modelValue', value);
    }
  },
});

const closeDialog = (): void => {
  show.value = false;
};

const onSave = (): void => {
  closeDialog();
  emit('saved');

  useAppStore().sendNotification({
    type: 'success',
    message:
      props.mode === 'add'
        ? i18n.global.t('wallets.add_account_proposal_saved')
        : i18n.global.t('wallets.edit_account_proposal_saved'),
  });
};

const submitted = ref(false);
const loading = ref(false);
const hasChanges = ref(false);

const submit = (): void => {
  submitted.value = true;
};
</script>
