<template>
  <VContainer>
    <VRow>
      <VCol cols="6">
        <BrandLogo variation="dark" />
      </VCol>
      <VCol cols="6">
        <div class="side-actions">
          <VBtn :icon="mdiBellRing" variant="text" size="small" />
        </div>
      </VCol>
      <VCol cols="12">
        <div class="profile">
          <VAvatar color="primary-variant" size="64" image="/images/avatar.svg" />
          <VBtn class="profile__username" variant="text" :append-icon="mdiChevronDown" size="small">
            <span>{{ auth.username ? auth.username : $t('terms.anonymous') }}</span>
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
import { useAuthStore } from '~/ui/stores';
import { mdiBellRing, mdiChevronDown } from '@mdi/js';
import BrandLogo from '~/ui/components/BrandLogo.vue';

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

  &__username {
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
