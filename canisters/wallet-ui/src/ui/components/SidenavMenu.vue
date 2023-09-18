<template>
  <VContainer>
    <VRow>
      <VCol cols="12" class="menu">
        <section class="menu__main">
          <template v-for="section in $navigation.main" :key="section.name">
            <div class="menu__title">{{ $t(section.localeKey) }}</div>
            <VDivider />
            <VList nav density="compact" class="ps-0 pe-0">
              <VListItem
                v-for="item in section.items"
                :key="item.name"
                :exact="true"
                :title="$t(item.localeKey)"
                :value="item.name"
                :to="isTo(item.action) ? item.action.handle($route) : undefined"
                :href="isHref(item.action) ? item.action.handle() : undefined"
                :prepend-icon="item.icon"
                @click="isCallback(item.action) ? item.action.handle() : undefined"
              />
            </VList>
          </template>
        </section>
      </VCol>
    </VRow>
  </VContainer>
</template>

<script lang="ts" setup>
import {
  NavigationAction,
  NavigationActionType,
  NavigationCallback,
  NavigationHref,
  NavigationTo,
} from '~/ui/types';

const isHref = (action: NavigationAction): action is NavigationHref => {
  return action.type === NavigationActionType.Href;
};

const isCallback = (action: NavigationAction): action is NavigationCallback => {
  return action.type === NavigationActionType.Callback;
};

const isTo = (action: NavigationAction): action is NavigationTo => {
  return action.type === NavigationActionType.To;
};
</script>

<style scoped lang="scss">
.menu {
  display: flex;
  flex-direction: column;

  &__title {
    padding: var(--ds-bdu) 0;
    font-size: var(--ds-font-size-xs);
    font-weight: bold;
  }
}
</style>
