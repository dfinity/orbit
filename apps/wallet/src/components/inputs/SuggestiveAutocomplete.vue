<template>
  <div class="d-flex flex-row flex-nowrap ga-4">
    <div v-if="props.icon" class="d-flex">
      <VIcon :icon="props.icon" class="text-medium-emphasis" />
    </div>
    <div class="d-flex flex-column flex-grow-1 ga-2">
      <div class="d-flex flex-column flex-grow-1 ga-1">
        <span v-if="props.label" class="text-medium-emphasis">{{ props.label }}</span>
        <div class="d-flex ga-2 flex-row flex-wrap">
          <VChip v-for="(item, idx) in selectedItems" :key="idx" size="small">
            {{ item.text }}
            <template v-if="!props.readonly" #append>
              <VBtn
                :icon="mdiCloseCircle"
                size="small"
                variant="text"
                density="compact"
                class="ml-2"
                @click.stop="unselectItem(item)"
              />
            </template>
          </VChip>
        </div>
      </div>
      <div v-if="!props.readonly" class="d-flex flex-nowrap ga-4">
        <VMenu>
          <template #activator="{ props: menuProps }">
            <VTextField
              v-model="search"
              v-bind="menuProps"
              :placeholder="props.placeholder"
              :name="props.name"
              :variant="props.variant"
              hide-details
              :density="props.density"
              @keydown.enter.stop.prevent="
                props.create
                  ? selectItem({ text: search, value: props.transform(search) })
                  : undefined
              "
            />
          </template>
          <VList v-if="showDropdown" density="compact">
            <VListItem v-if="loading" class="text-center">
              <VProgressCircular indeterminate size="24" />
            </VListItem>
            <template v-else>
              <VListItem
                v-for="(item, idx) in unselectedItems"
                :key="idx"
                @click="selectItem(item, false)"
              >
                <VListItemTitle class="d-flex flex-nowrap ga-2">
                  <div class="flex-grow-1">{{ item.text }}</div>
                  <div><VIcon :icon="mdiPlusCircle" size="x-small" /></div>
                </VListItemTitle>
              </VListItem>
              <VListItem
                v-if="!unselectedItems.length"
                class="text-center"
                @click="
                  search.length ? selectItem({ text: search, value: transform(search) }) : undefined
                "
              >
                <VListItemTitle v-if="search.length && !props.create">
                  {{ $t('app.no_matching_results', { search }) }}
                </VListItemTitle>
                <VListItemTitle v-else-if="search.length" class="d-flex flex-nowrap ga-2">
                  <div class="flex-grow-1">{{ $t('app.add_new_label', { label: search }) }}</div>
                  <div><VIcon :icon="mdiPlusCircle" size="x-small" /></div>
                </VListItemTitle>
                <VListItemTitle v-else>{{ $t('app.no_data') }}</VListItemTitle>
              </VListItem>
            </template>
          </VList>
        </VMenu>
        <VBtn
          v-if="props.create"
          data-test-id="create-item-btn"
          color="primary"
          :icon="mdiPlus"
          variant="tonal"
          :disabled="!canAddNewItem"
          size="small"
          @click.stop="selectItem({ text: search, value: transform(search) })"
        />
      </div>
    </div>
  </div>
</template>
<script lang="ts" setup generic="V">
import { mdiCloseCircle, mdiPlus, mdiPlusCircle } from '@mdi/js';
import { Ref, computed, ref, watch } from 'vue';
import {
  VBtn,
  VChip,
  VIcon,
  VList,
  VListItem,
  VListItemTitle,
  VMenu,
  VProgressCircular,
  VTextField,
} from 'vuetify/components';
import logger from '~/core/logger.core';
import { SelectItem } from '~/types/helper.types';
import { throttle } from '~/utils/helper.utils';

const props = withDefaults(
  defineProps<{
    /**
     * The selected items, if there is no matching items entry will be added as a custom item.
     */
    modelValue?: V[];
    /**
     * Whether the input is readonly, defaults to false.
     */
    readonly?: boolean;
    /**
     * Whether to allow creating custom items, defaults to true.
     */
    create?: boolean;
    label?: string;
    name?: string;
    placeholder?: string;
    icon?: string;
    /**
     * The items to be displayed in the dropdown.
     */
    items?: SelectItem<V>[];
    /**
     * The function to transform custom item text to the item value, defaults to the text as the value.
     *
     * Only applicable when `create` is true.
     */
    transform?: (item: string) => V;
    /**
     * The function to fetch items based on the search term, defaults to fetch items from the `items` prop.
     */
    fetchItems?: (term: string) => Promise<SelectItem<V>[]> | SelectItem<V>[];
    /**
     * The placeholder text for the input.
     */
    variant?: 'underlined' | 'outlined' | 'filled';
    density?: 'comfortable' | 'compact' | 'default';
  }>(),
  {
    modelValue: () => [],
    readonly: false,
    create: true,
    label: undefined,
    name: undefined,
    icon: undefined,
    placeholder: undefined,
    variant: 'filled',
    density: 'compact',
    items: () => [],
    transform: (item: string) => item as unknown as V,
    fetchItems: undefined,
  },
);

const emit = defineEmits<{
  (event: 'update:modelValue', payload: V[]): void;
  (event: 'update:items', payload: SelectItem[]): void;
}>();

const model = computed({
  get: () => props.modelValue,
  set: value => emit('update:modelValue', value),
});

const hasSearchValue = computed(() => search.value && search.value.trim().length > 0);
const noDataItems = ref(props.items ?? []) as Ref<SelectItem<V>[]>;
const unselectedItems = ref([]) as Ref<SelectItem<V>[]>;
const selectedItems = ref([]) as Ref<SelectItem<V>[]>;
const search = ref<string>('');
const loading = ref(false);

const selectItem = (item: SelectItem<V>, clearSearch = true): void => {
  if (!item.value || model.value.find(selectedItem => selectedItem === item.value)) {
    return;
  }

  model.value = [...model.value, item.value];

  // Clears the search input after selecting an item
  if (clearSearch) {
    search.value = '';
  }
};

const unselectItem = (itemToRemove: SelectItem<V>): void => {
  model.value = model.value.filter((value, _) => value !== itemToRemove.value);
};

const lookupItems = async (term: string): Promise<SelectItem<V>[]> => {
  try {
    loading.value = true;

    if (props.fetchItems) {
      return await props.fetchItems(term);
    }

    if (!term) {
      // Reset items to the initial items when the search term is empty.
      return noDataItems.value;
    }

    return unselectedItems.value.filter(item =>
      item.text.toLowerCase().startsWith(term.toLowerCase()),
    );
  } catch (error) {
    logger.error('Failed to lookup items', error);

    return noDataItems.value;
  } finally {
    loading.value = false;
  }
};

const triggerSearch = throttle(async () => {
  try {
    const term = search.value;
    const fetchedItems = await lookupItems(term);

    updateUnselectedItems(fetchedItems);
  } catch (error) {
    logger.error('Failed to search items', error);
  }
}, 500);

const updateUnselectedItems = (items: SelectItem<V>[]): void => {
  // Removes selected items from the fetched items to avoid duplicates.
  const entries = items.filter(
    item => !selectedItems.value.find(selectedItem => selectedItem.value === item.value),
  );

  unselectedItems.value = entries.sort((a, b) => a.text.localeCompare(b.text));
};

const canAddNewItem = computed(
  () =>
    props.create &&
    hasSearchValue.value &&
    !selectedItems.value.find(item => item.text === search.value),
);
const showDropdown = computed(
  () => unselectedItems.value.length > 0 || (hasSearchValue.value && canAddNewItem.value),
);

watch(
  () => search.value,
  () => triggerSearch(),
);

watch(
  () => props.items,
  updatedItems => {
    // Initialize no data items when the items are available, this is helpful when
    // the items are fetched asynchronously and the component is mounted before the items are available.
    if (updatedItems.length && !noDataItems.value.length) {
      noDataItems.value = updatedItems;

      if (!unselectedItems.value.length) {
        updateUnselectedItems(updatedItems);
      }
    }
  },
  { immediate: true },
);

watch(
  () => model.value,
  (selection, previous) => {
    const allItems = Array.from(
      new Set([...unselectedItems.value, ...props.items, ...selectedItems.value]),
    );

    const removedValues =
      previous?.filter(
        previousItem => !selection.find(selectedItem => selectedItem === previousItem),
      ) ?? [];
    const itemsRemoved = allItems.filter(item => removedValues.includes(item.value));

    const entries: SelectItem<V>[] = [];
    for (const selectedItem of selection) {
      const item = allItems.find(item => item.value === selectedItem);

      if (item) {
        entries.push(item);
      } else {
        // if not found in the items list add it as a custom item with the value as the text.
        entries.push({ text: `${selectedItem}`, value: selectedItem });
      }
    }

    selectedItems.value = entries.sort((a, b) => a.text.localeCompare(b.text));
    unselectedItems.value = unselectedItems.value.filter(
      item => !selection.find(selectedItem => selectedItem === item.value),
    );

    if (!hasSearchValue.value) {
      unselectedItems.value = Array.from(new Set([...unselectedItems.value, ...itemsRemoved]));
    }

    if (!hasSearchValue.value && unselectedItems.value.length === 0) {
      updateUnselectedItems(noDataItems.value);
    }

    if (hasSearchValue.value) {
      triggerSearch();
    }
  },
  { immediate: true },
);

watch(
  () => unselectedItems.value,
  () => emit('update:items', unselectedItems.value),
);
</script>
