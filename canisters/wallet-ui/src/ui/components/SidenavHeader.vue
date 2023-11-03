<template>
  <VContainer>
    <VRow>
      <VCol cols="6">
        <BrandLogo variation="dark" />
      </VCol>
      <VCol cols="6">
        <div class="side-actions">
          <NotificationsPanelToggle />
        </div>
      </VCol>
      <VCol cols="12">
        <div class="profile">
          <VAvatar color="primary-variant" size="64" image="/images/avatar.svg" />
          <VBtn class="profile__name" variant="text" :append-icon="mdiChevronDown" size="small">
            <span>{{ auth.userName ? auth.userName : $t('terms.anonymous') }}</span>
            <VMenu activator="parent">
              <VList density="compact">
                <VListItem :exact="true" :to="`/${$route.params.locale}/settings`">
                  <VListItemTitle>{{ $t('terms.settings') }}</VListItemTitle>
                </VListItem>
                <VListItem @click="auth.signOut">
                  <VListItemTitle>{{ $t('navigation.configuration.items.logout') }}</VListItemTitle>
                </VListItem>
              </VList>
            </VMenu>
          </VBtn>
          <p v-if="activeBank.hasUser" class="profile__principal">
            <span>{{ activeBank.user.id }}</span>
            <VBtn
              class="wallet-card__subtitle__copy"
              size="x-small"
              variant="text"
              :icon="mdiContentCopy"
              @click="
                settings.copyToClipboard(activeBank.user.id, $t('banks.user_copied_to_clipboard'))
              "
            />
          </p>
        </div>
      </VCol>
    </VRow>
  </VContainer>
</template>

<script lang="ts" setup>
import { mdiChevronDown, mdiContentCopy } from '@mdi/js';
import BrandLogo from '~/ui/components/BrandLogo.vue';
import NotificationsPanelToggle from '~/ui/components/NotificationsPanelToggle.vue';
import { useActiveBankStore, useAuthStore, useSettingsStore } from '~/ui/stores';

const auth = useAuthStore();
const settings = useSettingsStore();
const activeBank = useActiveBankStore();
</script>

<style scoped lang="scss">
.side-actions {
  height: var(--ds-toolbar-height);
  display: flex;
  align-items: center;
  justify-content: end;
}

.profile {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;

  &__name {
    margin-top: var(--ds-bdu);
  }

  &__principal {
    font-size: var(--ds-font-size-xxs);
    white-space: nowrap;
    width: 90%;
    overflow: hidden;
    display: flex;
    flex-direction: row;
    align-items: center;

    & > span {
      overflow: hidden;
      text-overflow: ellipsis;
    }
  }
}
</style>
