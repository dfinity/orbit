<template>
  <VMenu>
    <template #activator="{ props: menuProps }">
      <VBtn :icon="accountIcon" v-bind="menuProps"></VBtn>
    </template>
    <VList density="compact">
      <VListSubheader v-if="wallet.user">
        {{ $t('terms.user_id') }}<br />
        <p>
          <span>{{ wallet.user.me.id }}</span>
          <VBtn
            size="x-small"
            variant="text"
            :icon="mdiContentCopy"
            @click="app.copyToClipboard(wallet.user.me.id, $t('wallets.user_copied_to_clipboard'))"
          />
        </p>
      </VListSubheader>
      <VListItem :exact="true" :to="`/${$route.params.locale}/my-settings`">
        <VListItemTitle>{{ $t('navigation.account_info_settings') }}</VListItemTitle>
      </VListItem>
      <VDivider />
      <VListItem @click="auth.signOut">
        <VListItemTitle>{{ $t('navigation.logout') }}</VListItemTitle>
      </VListItem>
    </VList>
  </VMenu>
</template>

<script lang="ts" setup>
import { mdiContentCopy, mdiAccountCircle, mdiAccountCircleOutline } from '@mdi/js';
import { computed } from 'vue';
import { useAppStore, useAuthStore, useWalletStore } from '~/ui/stores';

const auth = useAuthStore();
const app = useAppStore();
const wallet = useWalletStore();

const props = withDefaults(
  defineProps<{
    variant?: 'outlined' | 'filled';
  }>(),
  {
    variant: 'filled',
  },
);

const accountIcon = computed(() =>
  props.variant === 'outlined' ? mdiAccountCircleOutline : mdiAccountCircle,
);
</script>
