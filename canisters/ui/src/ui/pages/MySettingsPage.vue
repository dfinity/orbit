<template>
  <PageLayout>
    <template #main-header>
      <VContainer class="pt-8 pb-8 pl-8 pr-8" fluid>
        <VRow>
          <VCol cols="12">
            <h1 class="text-h4">{{ $t('pages.user_settings.title') }}</h1>
            <p>{{ $t('pages.user_settings.subtitle') }}</p>
          </VCol>
        </VRow>
      </VContainer>
    </template>
    <template #main-body>
      <VRow>
        <VCol cols="12">
          <VCard variant="flat" class="mx-8" :loading="!session.user">
            <VCardText v-if="session.user">
              <VCol cols="12" class="pb-0">
                <VTextField
                  v-model="session.user.principal"
                  :label="$t('app.user_id')"
                  variant="underlined"
                  readonly
                />
              </VCol>
              <VCol cols="12" class="text-h5 pt-1 pb-0">
                {{ $t('app.wallets') }}
              </VCol>
              <VCol cols="12">
                <VTable>
                  <thead>
                    <tr>
                      <th class="pl-0">{{ $t('terms.wallet_name') }}</th>
                      <th>{{ $t('terms.canister_id') }}</th>
                      <th>&nbsp;</th>
                    </tr>
                  </thead>
                  <tbody>
                    <tr v-for="(wallet, idx) of session.user?.wallets" :key="idx">
                      <td class="pl-0">{{ wallet.name ?? '-' }}</td>
                      <td>
                        {{ wallet.canisterId }}
                        <VChip v-if="wallet.main" density="compact" class="ml-2" color="success">
                          {{ $t('terms.main') }}
                        </VChip>
                      </td>
                      <td class="text-right">
                        <VBtn
                          v-if="session.user.wallets.length > 1"
                          :icon="mdiClose"
                          density="compact"
                          variant="flat"
                          size="medium"
                          class="mr-2"
                          @click="page.confirmRemoveWallet(Principal.fromText(wallet.canisterId))"
                        />
                        <VBtn
                          :icon="mdiPencil"
                          density="compact"
                          variant="flat"
                          size="medium"
                          @click="
                            page.editWallet({
                              name: wallet.name,
                              canisterId: wallet.canisterId,
                              main: wallet.main,
                            })
                          "
                        />
                      </td>
                    </tr>
                  </tbody>
                </VTable>
                <VDialog
                  v-model="page.removeWalletDialog.open"
                  persistent
                  transition="dialog-bottom-transition"
                  scrollable
                >
                  <VCard :loading="page.removeWalletDialog.saving">
                    <VToolbar dark color="primary">
                      <VToolbarTitle>
                        {{ $t('pages.user_settings.remove_associated_wallet') }}
                      </VToolbarTitle>
                      <VBtn
                        :icon="mdiClose"
                        variant="text"
                        dark
                        @click="page.closeRemoveWalletDialog"
                      />
                    </VToolbar>
                    <VCardText>
                      <p>
                        {{ $t('pages.user_settings.confirm_remove_associated_wallet') }}
                      </p>
                    </VCardText>
                    <VCardActions class="px-6">
                      <VSpacer />
                      <VBtn
                        :loading="page.removeWalletDialog.saving"
                        color="primary-variant"
                        variant="flat"
                        type="submit"
                        @click="page.removeWallet"
                      >
                        {{ $t('app.confirm') }}
                      </VBtn>
                    </VCardActions>
                  </VCard>
                </VDialog>
                <VDialog
                  v-model="page.walletEditDialog.open"
                  persistent
                  transition="dialog-bottom-transition"
                  scrollable
                >
                  <VForm ref="form" @submit.prevent="saveWallet">
                    <VCard :loading="page.walletEditDialog.saving">
                      <VToolbar dark color="primary">
                        <VToolbarTitle>
                          {{ $t('pages.user_settings.manage_associated_wallet') }}
                        </VToolbarTitle>
                        <VBtn
                          :icon="mdiClose"
                          variant="text"
                          dark
                          @click="page.closeWalletEditDialog"
                        />
                      </VToolbar>
                      <VCardText>
                        <VTextField
                          v-model="page.walletEditDialog.fields.name"
                          :label="$t('terms.wallet_name')"
                          variant="underlined"
                          :rules="page.walletValidationRules.walletName"
                        />
                        <VSwitch
                          :label="$t('terms.main')"
                          inset
                          color="success"
                          hide-details
                          :model-value="page.walletEditDialog.fields.main"
                          @change="
                            page.walletEditDialog.fields.main = !page.walletEditDialog.fields.main
                          "
                        />
                      </VCardText>
                      <VCardActions class="px-6">
                        <small
                          >* {{ $t('pages.user_settings.manage_associated_wallet_hint') }}</small
                        >
                        <VSpacer />
                        <VBtn
                          :disabled="!page.walletHasChanges"
                          :loading="page.walletEditDialog.saving"
                          color="primary-variant"
                          variant="flat"
                          type="submit"
                          @click="saveWallet"
                        >
                          {{ $t('forms.edit') }}
                        </VBtn>
                      </VCardActions>
                    </VCard>
                  </VForm>
                </VDialog>
              </VCol>
            </VCardText>
          </VCard>
        </VCol>
      </VRow>
    </template>
  </PageLayout>
</template>

<script lang="ts" setup>
import { mdiPencil, mdiClose } from '@mdi/js';
import PageLayout from '~/ui/components/PageLayout.vue';
import { useSessionStore, useUserSettingsPage } from '~/ui/stores';
import { ref } from 'vue';
import { Principal } from '@dfinity/principal';

const session = useSessionStore();
const page = useUserSettingsPage();

const form = ref<{ validate: () => Promise<{ valid: boolean }> } | null>(null);

const saveWallet = async () => {
  const { valid } = form.value ? await form.value.validate() : { valid: false };

  page.walletEditDialog.isValid = valid;
  if (valid) {
    await page.saveWallet();
  }
};
</script>
