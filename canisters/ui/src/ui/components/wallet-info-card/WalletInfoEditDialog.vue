<template>
  <VDialog
    v-model="store.editDialog.open"
    persistent
    transition="dialog-bottom-transition"
    scrollable
  >
    <VForm ref="form" @submit.prevent="save">
      <VCard :loading="store.$state.editDialog.loading">
        <VToolbar dark color="primary">
          <VToolbarTitle>
            {{ $t('app.manage_associated_wallet') }}
          </VToolbarTitle>
          <VBtn :icon="mdiClose" variant="text" dark @click="store.closeEditDialog" />
        </VToolbar>
        <VCardText>
          <VTextField
            v-model="store.$state.editDialog.form.name"
            :label="$t('terms.wallet_name')"
            variant="underlined"
            :rules="store.validationRules.walletName"
          />
          <VSwitch
            :label="$t('terms.main')"
            inset
            color="success"
            hide-details
            :model-value="store.$state.editDialog.form.main"
            @change="store.$state.editDialog.form.main = !store.$state.editDialog.form.main"
          />
        </VCardText>
        <VCardActions class="px-6">
          <small>* {{ $t('app.manage_associated_wallet_hint') }}</small>
          <VSpacer />
          <VBtn
            :disabled="!store.canSave"
            :loading="store.$state.editDialog.loading"
            color="primary-variant"
            variant="flat"
            type="submit"
          >
            {{ $t('forms.edit') }}
          </VBtn>
        </VCardActions>
      </VCard>
    </VForm>
  </VDialog>
</template>

<script lang="ts" setup>
import { mdiClose } from '@mdi/js';
import { ref, computed, watch } from 'vue';
import type { VFormValidation } from '~/ui/types';
import { useStore } from './store';

const store = useStore();
const form = ref<VFormValidation | null>(null);

const isFormValid = computed(() => {
  return form.value ? form.value.isValid : false;
});

watch(
  () => isFormValid.value,
  isValid => {
    store.$state.editDialog.isValid = isValid;
  },
);

const save = async () => {
  const { valid } = form.value ? await form.value.validate() : { valid: false };

  store.editDialog.isValid = valid;
  if (store.canSave) {
    await store.saveChanges();
  }
};
</script>
