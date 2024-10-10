<template>
  <VMenu>
    <template #activator="{ props: menuProps }">
      <VBtn :icon="accountIcon" v-bind="menuProps"></VBtn>
    </template>
    <VList density="compact">
      <VListSubheader v-if="session.isAuthenticated">
        {{ $t('terms.identity') }}<br />
        <p>
          <TextOverflow :text="session.principal" :max-length="32" />
          <VBtn
            size="x-small"
            variant="text"
            :icon="mdiContentCopy"
            @click="
              copyToClipboard({
                textToCopy: session.principal,
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
      <VListItem @click="() => session.signOut()">
        <VListItemTitle>{{ $t('navigation.logout') }}</VListItemTitle>
      </VListItem>
    </VList>
  </VMenu>
</template>

<script lang="ts" setup>
import { mdiAccountCircle, mdiAccountCircleOutline, mdiContentCopy } from '@mdi/js';
import { computed } from 'vue';
import TextOverflow from '~/components/TextOverflow.vue';
import { useSessionStore } from '~/stores/session.store';
import { copyToClipboard } from '~/utils/app.utils';

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
