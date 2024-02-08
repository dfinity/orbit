<template>
  <VListItem
    v-if="!isGroup"
    :exact="props.item.exact === undefined ? false : props.item.exact"
    :value="props.item.name"
    :to="isTo(props.item.action) ? props.item.action.handle($route) : undefined"
    :href="isHref(props.item.action) ? props.item.action.handle() : undefined"
    @click="isCallback(props.item.action) ? props.item.action.handle() : undefined"
  >
    <VListItemTitle>
      <VIcon
        :icon="props.item.icon ? props.item.icon : mdiCircleSmall"
        color="secondary"
        class="mr-2"
      />
      {{ $t(props.item.localeKey) }}
    </VListItemTitle>
  </VListItem>
  <template v-else>
    <VListGroup :value="props.item.name" fluid>
      <template #activator="{ props: listProps }">
        <VListItem
          v-bind="listProps"
          :exact="props.item.exact === undefined ? false : props.item.exact"
          :value="props.item.name"
          :to="isTo(props.item.action) ? props.item.action.handle($route) : undefined"
          :href="isHref(props.item.action) ? props.item.action.handle() : undefined"
          @click="isCallback(props.item.action) ? props.item.action.handle() : undefined"
        >
          <VListItemTitle>
            <VIcon
              :icon="props.item.icon ? props.item.icon : mdiCircleSmall"
              color="secondary"
              class="mr-2"
            />
            {{ $t(props.item.localeKey) }}
          </VListItemTitle>
        </VListItem>
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
} from '~/types/navigation.types';

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
