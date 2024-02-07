<template>
  <VForm ref="form" class="account" @submit.prevent="">
    <VAlert
      v-if="accountForm.alert.show"
      :type="accountForm.alert.type"
      class="mx-4 mb-4"
      variant="tonal"
      density="compact"
    >
      {{ accountForm.alert.message }}
    </VAlert>
    <VContainer fluid>
      <VRow>
        <VCol v-if="!isViewMode" cols="12" class="account__title mb-4">
          <VBtn
            size="small"
            color="primary-variant"
            :variant="!accountForm.multiCustody ? 'tonal' : 'outlined'"
            class="mr-2"
            :prepend-icon="mdiAccount"
            @click="accountForm.multiCustody = false"
          >
            {{ $t('wallets.private_account') }}
          </VBtn>
          <VBtn
            size="small"
            color="primary-variant"
            :variant="accountForm.multiCustody ? 'tonal' : 'outlined'"
            :prepend-icon="mdiAccountGroup"
            @click="accountForm.multiCustody = true"
          >
            {{ $t('wallets.joint_account') }}
          </VBtn>
        </VCol>
        <VCol cols="12" class="py-0">
          <VTextField
            v-model="accountForm.form.name"
            :label="$t('terms.name')"
            variant="solo"
            density="compact"
            :readonly="isViewMode"
            :clearable="!isViewMode"
            :prepend-icon="mdiWallet"
            :rules="accountForm.validationRules.name"
          />
        </VCol>
        <VCol cols="12" class="py-0">
          <VAutocomplete
            v-model="accountForm.form.blockchain"
            :label="$t('terms.asset')"
            variant="solo"
            density="compact"
            :readonly="isViewMode"
            :clearable="!isViewMode"
            :prepend-icon="mdiKeyChainVariant"
            :rules="accountForm.validationRules.blockchain"
            :items="accountForm.supportedBlockchains"
          />
        </VCol>
        <template v-if="accountForm.multiCustody">
          <VCol cols="12" class="account__title">{{ $t('terms.owners') }}</VCol>
          <VCol v-for="(ownerId, idx) in accountForm.form.owners" :key="idx" cols="12" sm="4">
            <VCard density="compact" variant="elevated">
              <VCardText class="pb-0">
                <VTextField
                  v-model="accountForm.form.owners[idx]"
                  :prepend-icon="mdiAccount"
                  :label="$t('terms.user_id')"
                  :readonly="isViewMode"
                  variant="filled"
                  density="compact"
                  :rules="[
                    ...accountForm.validationRules.ownerUser,
                    uniqueRule(accountForm.form.owners.filter((_, self) => self !== idx)),
                  ]"
                  :clearable="!accountForm.isSelfOwnerEntry(ownerId)"
                  :disabled="accountForm.isSelfOwnerEntry(ownerId)"
                />
              </VCardText>
              <VCardActions v-if="!isViewMode">
                <VSpacer />
                <VBtn
                  color="error"
                  variant="text"
                  :disabled="accountForm.isSelfOwnerEntry(ownerId)"
                  @click="accountForm.removeOwnerByIndex(idx)"
                >
                  {{ $t('terms.remove') }}
                </VBtn>
              </VCardActions>
            </VCard>
          </VCol>
          <VCol v-if="accountForm.canAddOwner && !isViewMode" cols="12" md="4">
            <VCard density="compact" variant="plain" class="account__add">
              <VCardText class="text-center">
                <VIcon :icon="mdiAccount" size="64" />
              </VCardText>
              <VCardActions>
                <VSpacer />
                <VBtn color="success" variant="flat" block @click="accountForm.addOwner(null)">
                  {{ $t('terms.add') }}
                </VBtn>
                <VSpacer />
              </VCardActions>
            </VCard>
          </VCol>
          <VCol cols="12" class="account__title">{{ $t('terms.policies') }}</VCol>
        </template>
      </VRow>
    </VContainer>
  </VForm>
</template>
<script lang="ts" setup>
import { mdiAccount, mdiAccountGroup, mdiKeyChainVariant, mdiWallet } from '@mdi/js';
import { ref, computed, onMounted, watch } from 'vue';
import { uniqueRule } from '~/utils/form.utils';
import { Account, Proposal, UUID } from '~/generated/wallet/wallet.did';
import { useAccountForm } from './AccountForm.store';

const form = ref<{ validate: () => Promise<{ valid: boolean }> } | null>(null);
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

const accountForm = useAccountForm(props.account);

const emit = defineEmits<{
  (event: 'update:modelValue', payload: boolean): void;
  (event: 'saved', payload: Proposal): void;
  (event: 'loading', payload: boolean): void;
  (event: 'updated', payload: boolean): void;
}>();

const submitted = computed({
  get: () => props.modelValue,
  set: value => emit('update:modelValue', value),
});

const isViewMode = computed(() => props.mode === 'view');

onMounted(() => {
  accountForm.$subscribe(() => {
    if (accountForm.hasChanges) {
      emit('updated', true);
    }
  });
});

watch(submitted, () => {
  if (submitted.value) {
    submit();
  }

  submitted.value = false;
});

const submit = async () => {
  const { valid } = form.value ? await form.value.validate() : { valid: false };

  accountForm.isValid = valid;

  if (valid) {
    accountForm.loading = true;
    emit('loading', accountForm.loading);
    const proposal = await accountForm.save();
    if (proposal) {
      emit('saved', proposal);
    }
    accountForm.loading = false;
    emit('loading', accountForm.loading);
  }
};

accountForm.$subscribe((_, state) => {
  const uniqOwners: Array<UUID | null> = [];
  state.form.owners.forEach(ownerId => {
    if (!uniqOwners.find(id => id === ownerId)) {
      uniqOwners.push(ownerId);
    }
  });

  if (uniqOwners.length !== state.form.owners.length) {
    accountForm.form.owners = [...uniqOwners];
  }
});
</script>

<style scoped lang="scss">
.account {
  height: 100%;

  &__title {
    font-weight: bold;
  }

  &__add {
    display: flex;
    flex-direction: column;
    height: 100%;

    > .v-card-text {
      flex-grow: 1;
      display: flex;
      align-items: center;
      justify-content: center;
    }
  }
}
</style>
