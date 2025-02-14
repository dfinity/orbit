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
      <div class="d-flex flex-row flex-grow-0 pa-4 ga-6">
        <slot name="footer">
          <a v-if="showFooter" href="https://internetcomputer.org" target="_blank">
            <PoweredByImage width="180" alt="Internet Computer" draggable="false" />
          </a>
          <VImg :src="betaImage" height="24" />
        </slot>
      </div>
    </div>
  </VNavigationDrawer>
</template>

<script lang="ts" setup>
import { computed } from 'vue';
import { VNavigationDrawer } from 'vuetify/components';
import SidebarHighlights from '~/components/ui/SidebarHighlights.vue';
import { useAppStore } from '~/stores/app.store';
import SidenavHeader from './sidebar/SidenavHeader.vue';
import SidenavMenu from './sidebar/SidenavMenu.vue';
import PoweredByImage from '../ui/svg/PoweredByImage.vue';
import betaImage from '~assets/images/beta.png';

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
