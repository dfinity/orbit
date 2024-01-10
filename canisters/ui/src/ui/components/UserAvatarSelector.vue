<template>
  <VMenu>
    <template #activator="{ props: menuProps }">
      <VBtn :icon="accountIcon" v-bind="menuProps"></VBtn>
    </template>
    <VList density="compact">
      <VListSubheader v-if="session.isAuthenticated">
        {{ $t('terms.user_id') }}<br />
        <p>
          <span>{{ session.user.principal }}</span>
          <VBtn
            size="x-small"
            variant="text"
            :icon="mdiContentCopy"
            @click="
              copyToClipboard({
                textToCopy: session.user.principal,
                sendNotification: true,
              })
            "
          />
        </p>
      </VListSubheader>
      <VListItem :exact="true" :to="`/${$route.params.locale}/my-settings`">
        <VListItemTitle>{{ $t('navigation.account_info_settings') }}</VListItemTitle>
      </VListItem>
      <VDivider />
      <VListItem @click="session.signOut">
        <VListItemTitle>{{ $t('navigation.logout') }}</VListItemTitle>
      </VListItem>
    </VList>
  </VMenu>
</template>

<script lang="ts" setup>
import { mdiContentCopy, mdiAccountCircle, mdiAccountCircleOutline } from '@mdi/js';
import { computed } from 'vue';
import { useSessionStore } from '~/ui/stores/session';
import { copyToClipboard } from '~/ui/utils';

const session = useSessionStore();

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
