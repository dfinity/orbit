<template>
  <VNavigationDrawer
    v-model="app.showSidebar"
    class="sidebar"
    :width="props.width"
    :color="props.color"
  >
    <div class="d-flex flex-column h-100">
      <div class="sidebar__header d-flex flex-grow-0">
        <slot name="header">
          <SidenavHeader v-if="showHeader" />
        </slot>
      </div>
      <div class="sidebar_nav d-flex flex-grow-1">
        <slot name="nav">
          <SidenavMenu v-if="showNav" />
        </slot>
      </div>
      <div class="sidebar_footer d-flex flex-column flex-grow-0 pa-4">
        <slot name="footer">
          <AlphaWarning class="mb-4" />
          <a v-if="showFooter" href="https://internetcomputer.org" target="_blank">
            <img src="/images/powered-by-badge.svg" height="20" />
          </a>
        </slot>
      </div>
    </div>
  </VNavigationDrawer>
</template>

<script lang="ts" setup>
import { computed } from 'vue';
import { VNavigationDrawer } from 'vuetify/components';
import AlphaWarning from '~/components/layouts/AlphaWarning.vue';
import SidenavHeader from '~/components/SidenavHeader.vue';
import SidenavMenu from '~/components/SidenavMenu.vue';
import { useAppStore } from '~/stores/app.store';
import { useSessionStore } from '~/stores/session.store';

const props = withDefaults(
  defineProps<{
    color?: string;
    width?: number | string;
    header?: boolean;
    nav?: boolean;
    footer?: boolean;
  }>(),
  {
    color: 'sidebar',
    width: 260,
    header: true,
    nav: true,
    footer: true,
  },
);

const app = useAppStore();
const session = useSessionStore();

const showHeader = computed(() => props.header);
const showNav = computed(() => props.nav && session.isAuthenticated);
const showFooter = computed(() => props.footer);
</script>
