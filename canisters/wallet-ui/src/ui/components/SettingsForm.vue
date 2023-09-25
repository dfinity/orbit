<template>
  <VAlert
    v-if="settings.alert.show"
    :type="settings.alert.type"
    class="mx-4"
    variant="tonal"
    density="compact"
  >
    {{ settings.alert.message }}
  </VAlert>
  <VForm
    v-if="!settings.failedToLoad"
    ref="form"
    class="settings-form"
    @submit.prevent="saveChanges"
  >
    <VContainer>
      <VRow>
        <VCol cols="12 settings-form__title">{{ $t('terms.general') }}</VCol>
        <VCol cols="12" class="py-0">
          <VTextField
            v-model="auth.accountId"
            :prepend-inner-icon="mdiIdentifier"
            variant="solo"
            density="compact"
            readonly
          />
        </VCol>
        <VCol cols="12" class="py-0">
          <VTextField
            v-model="settings.form.name"
            :prepend-inner-icon="mdiAccount"
            :label="$t('terms.account_name')"
            variant="solo"
            density="compact"
            clearable
            :rules="settings.validationRules.accountName"
          />
        </VCol>
      </VRow>
      <VRow>
        <VCol cols="12" class="settings-form__title">
          {{
            $t('forms.banks', {
              min: settings.form.banks.length,
              max: 10,
            })
          }}
        </VCol>
        <VCol cols="12">
          <VRow>
            <VCol v-for="(bank, idx) of settings.form.banks" :key="idx" cols="12" md="4">
              <VCard density="compact" variant="elevated">
                <VCardText class="pb-0">
                  <VTextField
                    v-model="bank.name"
                    :prepend-inner-icon="mdiRename"
                    :label="$t('terms.bank_name')"
                    variant="filled"
                    density="compact"
                    clearable
                    :rules="settings.validationRules.bankName"
                  />
                  <VTextField
                    v-model="bank.canisterId"
                    :prepend-inner-icon="mdiIdentifier"
                    :label="$t('terms.canister_id')"
                    variant="filled"
                    density="compact"
                    :rules="[
                      ...settings.validationRules.validPrincipal,
                      uniqueRule(
                        settings.form.banks
                          .map(b => b.canisterId)
                          .filter((_, self) => self !== idx),
                      ),
                    ]"
                  />
                  <VSwitch
                    :label="$t('terms.main')"
                    inset
                    color="success"
                    hide-details
                    :model-value="bank.canisterId === settings.form.mainBank"
                    :readonly="bank.canisterId === settings.form.mainBank || !bank.canisterId"
                    @change="settings.toggleMainBank(bank.canisterId)"
                  />
                </VCardText>
                <VCardActions>
                  <VSpacer />
                  <VBtn color="error" variant="text" @click="settings.removeBank(idx)">
                    {{ $t('terms.remove') }}
                  </VBtn>
                </VCardActions>
              </VCard>
            </VCol>
            <VCol v-if="settings.canAddBank" cols="12" md="4">
              <VCard density="compact" variant="plain" class="settings-form__add">
                <VCardText class="text-center">
                  <VIcon :icon="mdiBank" size="64" />
                </VCardText>
                <VCardActions>
                  <VSpacer />
                  <VBtn color="success" variant="flat" block @click="settings.addBank">
                    {{ $t('terms.add') }}
                  </VBtn>
                  <VSpacer />
                </VCardActions>
              </VCard>
            </VCol>
          </VRow>
        </VCol>
      </VRow>
      <VRow>
        <VCol cols="12" class="settings-form__title">
          {{
            $t('forms.identities', {
              min: settings.form.identities.length,
              max: 10,
            })
          }}
        </VCol>
        <VCol cols="12">
          <VRow>
            <VCol v-for="(identity, idx) of settings.form.identities" :key="idx" cols="12" md="4">
              <VCard density="compact" variant="elevated">
                <VCardTitle class="d-flex pt-4">
                  <VSpacer />
                  <VChip :color="identity.confirmed ? 'success' : 'warning'">
                    {{ identity.confirmed ? $t('terms.confirmed') : $t('terms.unconfirmed') }}
                  </VChip>
                </VCardTitle>
                <VCardText class="pb-0">
                  <VTextField
                    v-model="identity.name"
                    :prepend-inner-icon="mdiRename"
                    :label="$t('terms.identity_name')"
                    variant="filled"
                    density="compact"
                    clearable
                    :disabled="!identity.confirmed"
                    :rules="settings.validationRules.identityName"
                  />
                  <VTextField
                    v-model="identity.principal"
                    :prepend-inner-icon="mdiIdentifier"
                    :label="$t('terms.principal')"
                    variant="filled"
                    density="compact"
                    :disabled="identity.principal === auth.principal"
                    :rules="[
                      ...settings.validationRules.validPrincipal,
                      uniqueRule(
                        settings.form.identities
                          .map(i => i.principal)
                          .filter((_, self) => self !== idx),
                      ),
                    ]"
                  />
                </VCardText>
                <VCardActions>
                  <VSpacer />
                  <VBtn
                    color="error"
                    variant="text"
                    :disabled="identity.principal === auth.principal"
                    @click="settings.removeIdentity(idx)"
                  >
                    {{ $t('terms.remove') }}
                  </VBtn>
                </VCardActions>
              </VCard>
            </VCol>
            <VCol v-if="settings.canAddIdentity" cols="12" md="4">
              <VCard density="compact" variant="plain" class="settings-form__add">
                <VCardText class="text-center">
                  <VIcon :icon="mdiAccountKey" size="64" />
                </VCardText>
                <VCardActions>
                  <VSpacer />
                  <VBtn color="success" variant="flat" block @click="settings.addIdentity">
                    {{ $t('terms.add') }}
                  </VBtn>
                  <VSpacer />
                </VCardActions>
              </VCard>
            </VCol>
          </VRow>
        </VCol>
      </VRow>
      <VRow>
        <VCol cols="12" class="d-flex">
          <VSpacer />
          <VBtn
            color="secondary-variant"
            :prepend-icon="mdiContentSave"
            :loading="settings.isLoading"
            :disabled="!settings.canSave"
            type="submit"
            block
          >
            {{ $t('forms.save_changes') }}
          </VBtn>
        </VCol>
      </VRow>
    </VContainer>
  </VForm>
</template>

<script lang="ts" setup>
import {
  mdiAccount,
  mdiAccountKey,
  mdiBank,
  mdiContentSave,
  mdiIdentifier,
  mdiRename,
} from '@mdi/js';
import { onMounted, ref } from 'vue';
import { useAuthStore, useSettingsFormStore } from '~/ui/stores';
import { uniqueRule } from '~/ui/utils';

const form = ref<{ validate: () => Promise<{ valid: boolean }> } | null>(null);
const auth = useAuthStore();
const settings = useSettingsFormStore();

const saveChanges = async () => {
  const { valid } = form.value ? await form.value.validate() : { valid: false };

  settings.isValid = valid;
  if (valid) {
    await settings.save();
  }
};

onMounted(() => settings.load());
</script>

<style scoped lang="scss">
.settings-form {
  &__title {
    font-weight: bold;
    font-size: var(--ds-font-size-xs);
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

  .v-field--variant-filled .v-field__overlay {
    opacity: 1;
  }
}
</style>

<style lang="scss">
.settings-form {
  .v-field--variant-filled .v-field__overlay {
    opacity: 0;
  }
}
</style>
