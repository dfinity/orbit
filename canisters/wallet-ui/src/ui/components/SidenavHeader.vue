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
            <span>{{ auth.accountName ? auth.accountName : $t('terms.anonymous') }}</span>
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
          <span class="profile__principal">{{ auth.accountId }}</span>
        </div>
      </VCol>
    </VRow>
  </VContainer>
</template>

<script lang="ts" setup>
import { mdiChevronDown } from '@mdi/js';
import BrandLogo from '~/ui/components/BrandLogo.vue';
import NotificationsPanelToggle from '~/ui/components/NotificationsPanelToggle.vue';
import { useAuthStore } from '~/ui/stores';

const auth = useAuthStore();
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
    text-overflow: ellipsis;
    white-space: nowrap;
    width: 80%;
    overflow: hidden;
  }
}
</style>
