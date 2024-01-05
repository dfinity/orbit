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
            <span>{{
              session.hasConnectedWalletUser && session.connectedWalletUser.me.name
                ? session.connectedWalletUser.me.name
                : $t('terms.anonymous')
            }}</span>
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
          <p v-if="session.hasConnectedWalletUser" class="profile__principal">
            <span>{{ session.connectedWalletUser.me.id }}</span>
            <VBtn
              size="x-small"
              variant="text"
              :icon="mdiContentCopy"
              @click="
                app.copyToClipboard(
                  session.connectedWalletUser.me.id,
                  $t('wallets.user_copied_to_clipboard'),
                )
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
import { useAppStore, useAuthStore, useSessionStore } from '~/ui/stores';

const auth = useAuthStore();
const app = useAppStore();
const session = useSessionStore();
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
