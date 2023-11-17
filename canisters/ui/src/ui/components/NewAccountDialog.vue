<template>
  <VDialog v-model="accountForm.show" persistent transition="dialog-bottom-transition" scrollable>
    <VForm ref="form" class="account-form" @submit.prevent="createAccount">
      <VCard :loading="accountForm.loading">
        <VToolbar dark color="primary">
          <VBtn icon dark @click="accountForm.close"><VIcon :icon="mdiClose" /></VBtn>
          <VToolbarTitle>
            {{ $t('terms.new_account') }}
          </VToolbarTitle>
        </VToolbar>
        <VCardText>
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
              <VCol cols="12" class="account-form__title mb-4">
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
                  clearable
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
                  clearable
                  :prepend-icon="mdiKeyChainVariant"
                  :rules="accountForm.validationRules.blockchain"
                  :items="accountForm.supportedBlockchains"
                />
              </VCol>
              <template v-if="accountForm.multiCustody">
                <VCol cols="12" class="account-form__title">{{ $t('terms.owners') }}</VCol>
                <VCol v-for="(ownerId, idx) in accountForm.form.owners" :key="idx" cols="12" sm="4">
                  <VCard density="compact" variant="elevated">
                    <VCardText class="pb-0">
                      <VTextField
                        v-model="accountForm.form.owners[idx]"
                        :prepend-icon="mdiAccount"
                        :label="$t('terms.user_id')"
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
                    <VCardActions>
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
                <VCol v-if="accountForm.canAddOwner" cols="12" md="4">
                  <VCard density="compact" variant="plain" class="account-form__add">
                    <VCardText class="text-center">
                      <VIcon :icon="mdiAccount" size="64" />
                    </VCardText>
                    <VCardActions>
                      <VSpacer />
                      <VBtn
                        color="success"
                        variant="flat"
                        block
                        @click="accountForm.addOwner(null)"
                      >
                        {{ $t('terms.add') }}
                      </VBtn>
                      <VSpacer />
                    </VCardActions>
                  </VCard>
                </VCol>
                <VCol cols="12" class="account-form__title">{{ $t('terms.policies') }}</VCol>
                <VCol v-for="(_, idx) in accountForm.form.policies" :key="idx" cols="12" md="4">
                  <AccountPolicyCard
                    v-model="accountForm.form.policies[idx]"
                    :owners="accountForm.nrOfOwners"
                    @removed="accountForm.removePolicyByIndex(idx)"
                  />
                </VCol>
                <VCol v-if="accountForm.canAddPolicy" cols="12" md="4">
                  <VCard density="compact" variant="plain" class="account-form__add">
                    <VCardText class="text-center">
                      <VIcon :icon="mdiCogs" size="64" />
                    </VCardText>
                    <VCardActions>
                      <VSpacer />
                      <VBtn color="success" variant="flat" block @click="accountForm.addNewPolicy">
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
          <VBtn variant="text" @click="accountForm.close">{{ $t('terms.close') }}</VBtn>
          <VBtn
            :disabled="!accountForm.hasChanges"
            :loading="accountForm.loading"
            color="primary"
            variant="flat"
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
import { useCreateAccountFormStore } from '~/ui/stores';
import { uniqueRule } from '~/ui/utils';
import AccountPolicyCard from './AccountPolicyCard.vue';
import { UserId, Policy } from '~/generated/wallet/wallet.did';

const form = ref<{ validate: () => Promise<{ valid: boolean }> } | null>(null);
const accountForm = useCreateAccountFormStore();

accountForm.$subscribe((_, state) => {
  const uniqOwners: Array<UserId | null> = [];
  state.form.owners.forEach(ownerId => {
    if (!uniqOwners.find(id => id === ownerId)) {
      uniqOwners.push(ownerId);
    }
  });

  if (uniqOwners.length !== state.form.owners.length) {
    accountForm.form.owners = [...uniqOwners];
  }

  const uniqPolicies: Map<string, Policy | null> = new Map();
  state.form.policies.forEach(entry => {
    uniqPolicies.set(JSON.stringify(entry), entry);
  });

  if (uniqPolicies.size !== state.form.policies.length) {
    accountForm.form.policies = [...uniqPolicies.values()];
  }
});

const createAccount = async (): Promise<void> => {
  const { valid } = form.value ? await form.value.validate() : { valid: false };

  accountForm.isValid = valid;
  if (valid) {
    await accountForm.save();
  }
};
</script>

<style scoped lang="scss">
.account-form {
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
