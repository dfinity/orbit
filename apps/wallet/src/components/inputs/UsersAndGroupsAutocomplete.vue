<template>
  <VAutocomplete
    v-if="initializing"
    :label="props.label"
    :readonly="true"
    :placeholder="loadingPlaceholder"
  >
    <template #append-inner>
      <VProgressCircular v-if="initializing || searching > 0" indeterminate :size="20" :width="2" />
    </template>
  </VAutocomplete>
  <VAutocomplete
    v-else
    v-model="model"
    :multiple="props.multiple"
    :label="props.label"
    :items="items"
    :readonly="readonly"
    :closable-chips="!readonly"
    :chips="true"
    :no-filter="true"
    item-title="text"
    item-value="value"
    :placeholder="props.placeholder"
    @update:search="searchItems"
  >
    <template #item="{ props: autocompleteProps, item }">
      <VListSubheader v-if="item.props.header">
        {{ item.props.header }}
      </VListSubheader>
      <VListItem v-else v-bind="autocompleteProps" :title="undefined">
        <VListItemTitle>{{ item.title }}</VListItemTitle>
      </VListItem>
    </template>
    <template #append-inner>
      <VProgressCircular v-if="initializing || searching > 0" indeterminate :size="20" :width="2" />
    </template>
  </VAutocomplete>
</template>
<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import {
  useUserGroupsAutocomplete,
  useUsersAutocomplete,
} from '~/composables/autocomplete.composable';
import { SelectItem } from '~/types/helper.types';
import { debounce } from '~/utils/helper.utils';

export interface UserAndGroupsAutocompleteModel {
  type: 'user' | 'group';
  id: string;
}

export type Item = { props: { header: string } } | SelectItem<UserAndGroupsAutocompleteModel>;

const props = withDefaults(
  defineProps<{
    multiple?: boolean;
    modelValue?: UserAndGroupsAutocompleteModel | UserAndGroupsAutocompleteModel[];
    label?: string;
    readonly?: boolean;
    loadingPlaceholder?: string;
    placeholder?: string;
  }>(),
  {
    multiple: false,
    modelValue: undefined,
    label: undefined,
    readonly: false,
    loadingPlaceholder: undefined,
    placeholder: undefined,
  },
);

const emit = defineEmits<{
  (
    event: 'update:modelValue',
    payload?: UserAndGroupsAutocompleteModel | UserAndGroupsAutocompleteModel[],
  ): void;
}>();

const i18n = useI18n();
const initializing = ref(true);
const items = ref<Item[]>([]);
const selectedItemsCache = ref<Record<string, string>>({});
const searching = ref(0);
const usersAutocomplete = useUsersAutocomplete();
const userGroupsAutocomplete = useUserGroupsAutocomplete();
const readonly = computed(() => props.readonly || initializing.value);
const loadingPlaceholder = computed(
  () => props.loadingPlaceholder || `${i18n.t('terms.loading')}...`,
);

const model = computed({
  get: () => props.modelValue,
  set: value => emit('update:modelValue', value),
});

const searchItems = debounce(async (term?: string): Promise<void> => {
  searching.value++;

  const users = usersAutocomplete.searchItems(term);
  const groups = userGroupsAutocomplete.searchItems(term);

  return Promise.all([users, groups])
    .then(_ => {
      updateAvailableItemsList({
        users: usersAutocomplete.results.value.map(u => ({
          value: { type: 'user', id: u.id },
          text: u.name,
        })),
        groups: userGroupsAutocomplete.results.value.map(g => ({
          value: { type: 'group', id: g.id },
          text: g.name,
        })),
      });
    })
    .finally(() => {
      searching.value--;
    });
}, 750);

const availableUsers = computed<Record<string, string>>(() =>
  usersAutocomplete.results.value.reduce((acc: Record<string, string>, user) => {
    acc[user.id] = user.name;
    return acc;
  }, {}),
);

const availableGroups = computed<Record<string, string>>(() =>
  userGroupsAutocomplete.results.value.reduce((acc: Record<string, string>, group) => {
    acc[group.id] = group.name;
    return acc;
  }, {}),
);

const doCacheSelectedItems = () => {
  const currentPermitted = model.value ? (Array.isArray(model.value) ? model.value : []) : [];

  for (const permittedItem of currentPermitted) {
    switch (permittedItem.type) {
      case 'user':
        selectedItemsCache.value[permittedItem.id] = availableUsers.value[permittedItem.id];
        break;
      case 'group':
        selectedItemsCache.value[permittedItem.id] = availableGroups.value[permittedItem.id];
        break;
    }
  }
};

// Upon selection change, cache the selected items to display them as chips in the autocomplete even
// if they are not available in the list of available items anymore.
watch(model, _ => doCacheSelectedItems());

const updateAvailableItemsList = (
  results: {
    users: SelectItem<UserAndGroupsAutocompleteModel>[];
    groups: SelectItem<UserAndGroupsAutocompleteModel>[];
  } = { users: [], groups: [] },
) => {
  const currentPermitted = model.value ? (Array.isArray(model.value) ? model.value : []) : [];
  const permittedUsers = currentPermitted.filter(permitted => permitted.type === 'user');
  const permittedGroups = currentPermitted.filter(permitted => permitted.type === 'group');
  for (const permittedUser of permittedUsers) {
    const found = results.users.find(user => user.value.id === permittedUser.id);
    if (!found) {
      results.users.push({
        value: { type: 'user', id: permittedUser.id },
        text: selectedItemsCache.value[permittedUser.id] || permittedUser.id,
      });
    }
  }

  for (const permittedGroup of permittedGroups) {
    const found = results.groups.find(group => group.value.id === permittedGroup.id);
    if (!found) {
      results.groups.push({
        value: { type: 'group', id: permittedGroup.id },
        text: selectedItemsCache.value[permittedGroup.id] || permittedGroup.id,
      });
    }
  }

  doCacheSelectedItems();

  items.value = [
    ...(results.groups.length > 0
      ? [
          { props: { header: i18n.t('terms.user_groups') } },
          ...results.groups.sort((a, b) => a.text.localeCompare(b.text)),
        ]
      : []),
    ...(results.users.length > 0
      ? [
          { props: { header: i18n.t('terms.users') } },
          ...results.users.sort((a, b) => a.text.localeCompare(b.text)),
        ]
      : []),
  ];
};

onMounted(() => {
  initializing.value = true;

  updateAvailableItemsList();
  searchItems().finally(() => {
    initializing.value = false;
  });
});
</script>
