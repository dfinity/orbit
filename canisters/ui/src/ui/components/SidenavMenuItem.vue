<template>
  <VListItem
    v-if="!isGroup"
    :exact="true"
    :title="$t(props.item.localeKey)"
    :value="props.item.name"
    :to="isTo(props.item.action) ? props.item.action.handle($route) : undefined"
    :href="isHref(props.item.action) ? props.item.action.handle() : undefined"
    :prepend-icon="props.item.icon ? props.item.icon : mdiCircleSmall"
    @click="isCallback(props.item.action) ? props.item.action.handle() : undefined"
  />
  <template v-else>
    <VListGroup :value="props.item.name" fluid>
      <template #activator="{ props: listProps }">
        <VListItem
          v-bind="listProps"
          :exact="true"
          :title="$t(props.item.localeKey)"
          :value="props.item.name"
          :to="isTo(props.item.action) ? props.item.action.handle($route) : undefined"
          :href="isHref(props.item.action) ? props.item.action.handle() : undefined"
          :prepend-icon="props.item.icon ? props.item.icon : mdiCircleSmall"
          @click="isCallback(props.item.action) ? props.item.action.handle() : undefined"
        />
      </template>
      <SidenavMenuItem v-for="(menuItem, idx) in props.item.items" :key="idx" :item="menuItem" />
    </VListGroup>
    <VDivider />
  </template>
</template>

<script lang="ts" setup>
import { mdiCircleSmall } from '@mdi/js';
import { computed } from 'vue';
import {
  NavigationAction,
  NavigationActionType,
  NavigationCallback,
  NavigationHref,
  NavigationItem,
  NavigationTo,
} from '~/ui/types';

const props = defineProps<{
  item: NavigationItem;
}>();

const isHref = (action: NavigationAction): action is NavigationHref => {
  return action.type === NavigationActionType.Href;
};

const isCallback = (action: NavigationAction): action is NavigationCallback => {
  return action.type === NavigationActionType.Callback;
};

const isTo = (action: NavigationAction): action is NavigationTo => {
  return action.type === NavigationActionType.To;
};

const isGroup = computed(() => props.item.items && props.item.items.length > 0);
</script>
