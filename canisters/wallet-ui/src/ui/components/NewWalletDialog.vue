<template>
  <VDialog v-model="walletForm.show" persistent transition="dialog-bottom-transition" scrollable>
    <VForm ref="form" class="wallet-form" @submit.prevent="createWallet">
      <VCard :loading="walletForm.loading">
        <VToolbar dark color="primary">
          <VBtn icon dark @click="walletForm.close"><VIcon :icon="mdiClose" /></VBtn>
          <VToolbarTitle>
            {{ $t('terms.new_wallet') }}
          </VToolbarTitle>
        </VToolbar>
        <VCardText>
          <VAlert
            v-if="walletForm.alert.show"
            :type="walletForm.alert.type"
            class="mx-4 mb-4"
            variant="tonal"
            density="compact"
          >
            {{ walletForm.alert.message }}
          </VAlert>
          <VContainer fluid>
            <VRow>
              <VCol cols="12" class="wallet-form__title mb-4">
                <VBtn
                  size="small"
                  color="primary-variant"
                  :variant="!walletForm.multiCustody ? 'tonal' : 'outlined'"
                  class="mr-2"
                  :prepend-icon="mdiAccount"
                  @click="walletForm.multiCustody = false"
                >
                  {{ $t('banks.private_wallet') }}
                </VBtn>
                <VBtn
                  size="small"
                  color="primary-variant"
                  :variant="walletForm.multiCustody ? 'tonal' : 'outlined'"
                  :prepend-icon="mdiAccountGroup"
                  @click="walletForm.multiCustody = true"
                >
                  {{ $t('banks.joint_wallet') }}
                </VBtn>
              </VCol>
              <VCol cols="12" class="py-0">
                <VTextField
                  v-model="walletForm.form.name"
                  :label="$t('terms.name')"
                  variant="solo"
                  density="compact"
                  clearable
                  :prepend-icon="mdiWallet"
                  :rules="walletForm.validationRules.name"
                />
              </VCol>
              <VCol cols="12" class="py-0">
                <VAutocomplete
                  v-model="walletForm.form.blockchain"
                  :label="$t('terms.asset')"
                  variant="solo"
                  density="compact"
                  clearable
                  :prepend-icon="mdiKeyChainVariant"
                  :rules="walletForm.validationRules.blockchain"
                  :items="walletForm.supportedBlockchains"
                />
              </VCol>
              <template v-if="walletForm.multiCustody">
                <VCol cols="12" class="wallet-form__title">{{ $t('terms.owners') }}</VCol>
                <VCol v-for="(entry, idx) in walletForm.form.owners" :key="idx" cols="12" sm="4">
                  <VCard density="compact" variant="elevated">
                    <VCardText class="pb-0">
                      <div class="mb-3">
                        <VBtn
                          :disabled="walletForm.isSelfOwnerEntry(entry)"
                          size="x-small"
                          color="primary-variant"
                          :variant="entry.type === 'account' ? 'tonal' : 'outlined'"
                          class="mr-2"
                          @click="changeOwnerEntryType(entry, 'account')"
                        >
                          {{ $t('terms.account') }}
                        </VBtn>
                        <VBtn
                          :disabled="walletForm.isSelfOwnerEntry(entry)"
                          size="x-small"
                          color="primary-variant"
                          :variant="entry.type === 'principal' ? 'tonal' : 'outlined'"
                          class="mr-2"
                          @click="changeOwnerEntryType(entry, 'principal')"
                        >
                          {{ $t('terms.principal') }}
                        </VBtn>
                      </div>
                      <VTextField
                        v-model="entry.id"
                        :prepend-icon="mdiAccount"
                        :label="
                          entry.type === 'account' ? $t('terms.account_id') : $t('terms.principal')
                        "
                        variant="filled"
                        density="compact"
                        :rules="
                          entry.type === 'account'
                            ? [
                                ...walletForm.validationRules.ownerAccount,
                                uniqueRule(
                                  walletForm.form.owners
                                    .filter(item => item.type === 'account')
                                    .map(item => item.id)
                                    .filter((_, self) => self !== idx),
                                ),
                              ]
                            : [
                                ...walletForm.validationRules.ownerIdentity,
                                uniqueRule(
                                  walletForm.form.owners
                                    .filter(item => item.type === 'principal')
                                    .map(item => item.id)
                                    .filter((_, self) => self !== idx),
                                ),
                              ]
                        "
                        :clearable="!walletForm.isSelfOwnerEntry(entry)"
                        :disabled="walletForm.isSelfOwnerEntry(entry)"
                      />
                    </VCardText>
                    <VCardActions>
                      <VSpacer />
                      <VBtn
                        color="error"
                        variant="text"
                        :disabled="walletForm.isSelfOwnerEntry(entry)"
                        @click="walletForm.removeOwnerByIndex(idx)"
                      >
                        {{ $t('terms.remove') }}
                      </VBtn>
                    </VCardActions>
                  </VCard>
                </VCol>
                <VCol v-if="walletForm.canAddOwner" cols="12" md="4">
                  <VCard density="compact" variant="plain" class="wallet-form__add">
                    <VCardText class="text-center">
                      <VIcon :icon="mdiAccount" size="64" />
                    </VCardText>
                    <VCardActions>
                      <VSpacer />
                      <VBtn
                        color="success"
                        variant="flat"
                        block
                        @click="walletForm.addOwner({ type: 'account', id: null })"
                      >
                        {{ $t('terms.add') }}
                      </VBtn>
                      <VSpacer />
                    </VCardActions>
                  </VCard>
                </VCol>
                <VCol cols="12" class="wallet-form__title">{{ $t('terms.policies') }}</VCol>
                <VCol v-for="(_, idx) in walletForm.form.policies" :key="idx" cols="12" md="4">
                  <WalletPolicyCard
                    v-model="walletForm.form.policies[idx]"
                    :owners="walletForm.nrOfOwners"
                    @removed="walletForm.removePolicyByIndex(idx)"
                  />
                </VCol>
                <VCol v-if="walletForm.canAddPolicy" cols="12" md="4">
                  <VCard density="compact" variant="plain" class="wallet-form__add">
                    <VCardText class="text-center">
                      <VIcon :icon="mdiCogs" size="64" />
                    </VCardText>
                    <VCardActions>
                      <VSpacer />
                      <VBtn color="success" variant="flat" block @click="walletForm.addNewPolicy">
                        {{ $t('terms.add') }}
                      </VBtn>
                      <VSpacer />
                    </VCardActions>
                  </VCard>
                </VCol>
              </template>
            </VRow>
          </VContainer>
        </VCardText>
        <VCardActions>
          <VSpacer />
          <VBtn variant="text" @click="walletForm.close">{{ $t('terms.close') }}</VBtn>
          <VBtn
            :disabled="!walletForm.hasChanges"
            :loading="walletForm.loading"
            color="primary"
            variant="tonal"
            type="submit"
          >
            {{ $t('forms.create') }}
          </VBtn>
        </VCardActions>
      </VCard>
    </VForm>
  </VDialog>
</template>

<script lang="ts" setup>
import {
  mdiAccount,
  mdiAccountGroup,
  mdiClose,
  mdiKeyChainVariant,
  mdiWallet,
  mdiCogs,
} from '@mdi/js';
import { ref } from 'vue';
import { WalletOwnerEntry, useCreateWalletFormStore } from '~/ui/stores';
import { uniqueRule } from '~/ui/utils';
import WalletPolicyCard from './WalletPolicyCard.vue';
import { WalletPolicy } from '~/generated/bank/bank.did';

const form = ref<{ validate: () => Promise<{ valid: boolean }> } | null>(null);
const walletForm = useCreateWalletFormStore();

const changeOwnerEntryType = (entry: WalletOwnerEntry, type: 'account' | 'principal'): void => {
  if (entry.type !== type) {
    entry.id = null;
  }
  entry.type = type;
};

walletForm.$subscribe((_, state) => {
  const uniqOwners: WalletOwnerEntry[] = [];
  state.form.owners.forEach(entry => {
    if (!uniqOwners.find(item => item.id === entry.id && item.type === entry.type)) {
      uniqOwners.push(Object.assign({}, entry));
    }
  });

  if (uniqOwners.length !== state.form.owners.length) {
    walletForm.form.owners = [...uniqOwners];
  }

  const uniqPolicies: Map<string, WalletPolicy | null> = new Map();
  state.form.policies.forEach(entry => {
    uniqPolicies.set(JSON.stringify(entry), entry);
  });

  if (uniqPolicies.size !== state.form.policies.length) {
    walletForm.form.policies = [...uniqPolicies.values()];
  }
});

const createWallet = async (): Promise<void> => {
  const { valid } = form.value ? await form.value.validate() : { valid: false };

  walletForm.isValid = valid;
  if (valid) {
    await walletForm.save();
  }
};
</script>

<style scoped lang="scss">
.wallet-form {
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
