<template>
  <VContainer class="py-2">
    <VRow>
      <VCol cols="12" class="d-flex flex-column">
        <section class="menu__main">
          <template v-for="(menuItem, _idx) in $navigation.value.main" :key="_idx">
            <VList nav density="compact" class="ps-0 pe-0">
              <SidenavMenuItem :item="menuItem" />
            </VList>
          </template>
        </section>
      </VCol>
    </VRow>
  </VContainer>
</template>

<script lang="ts" setup>
import { VCol, VContainer, VList, VRow } from 'vuetify/components';
import SidenavMenuItem from '~/components/SidenavMenuItem.vue';
</script>
