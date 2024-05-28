<template>
  <VNavigationDrawer v-model="app.showSidebar" :width="props.width" :color="props.color">
    <div class="d-flex flex-column h-100">
      <div class="d-flex flex-grow-0">
        <slot name="header">
          <SidenavHeader v-if="showHeader" :language-selector="props.languageSelector" />
        </slot>
      </div>
      <div class="d-flex flex-column flex-grow-1">
        <slot name="nav">
          <SidenavMenu v-if="showNav" />
          <SidebarHighlights v-if="props.nav && !$navigation.value.main.length" class="mb-4" />
        </slot>
      </div>
      <div class="d-flex flex-column flex-grow-0 pa-4">
        <slot name="footer">
          <AlphaWarning class="mb-12" />
          <a v-if="showFooter" href="https://internetcomputer.org" target="_blank">
            <img :src="poweredByBadge" height="20" />
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
import SidebarHighlights from '~/components/ui/SidebarHighlights.vue';
import { useAppStore } from '~/stores/app.store';
import poweredByBadge from '~assets/images/powered-by-badge.svg';
import SidenavHeader from './sidebar/SidenavHeader.vue';
import SidenavMenu from './sidebar/SidenavMenu.vue';

const props = withDefaults(
  defineProps<{
    color?: string;
    width?: number | string;
    header?: boolean;
    nav?: boolean;
    footer?: boolean;
    languageSelector?: boolean;
  }>(),
  {
    color: 'sidebar',
    width: 300,
    header: true,
    nav: true,
    footer: true,
    languageSelector: false,
  },
);

const app = useAppStore();

const showHeader = computed(() => props.header);
const showNav = computed(() => props.nav);
const showFooter = computed(() => props.footer);
</script>
