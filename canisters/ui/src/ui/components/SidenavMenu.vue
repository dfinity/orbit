<template>
  <VContainer>
    <VRow>
      <VCol cols="12" class="menu">
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
import SidenavMenuItem from '~/ui/components/SidenavMenuItem.vue';
</script>

<style scoped lang="scss">
.menu {
  display: flex;
  flex-direction: column;
  font-size: var(--ds-font-size-xxs);
}
</style>
