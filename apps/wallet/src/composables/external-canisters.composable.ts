import { Principal } from '@dfinity/principal';
import { ComputedRef, computed, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { LocationQuery, useRouter } from 'vue-router';
import { useAutocomplete } from '~/composables/autocomplete.composable';
import logger from '~/core/logger.core';
import { useStationStore } from '~/stores/station.store';
import { SelectItem } from '~/types/helper.types';
import { ExternalCanisterStateEnum } from '~/types/station.types';

export type ExternalCanistersFilters = {
  name_prefix: string;
  canisters: string[];
  canister_items: SelectItem<string>[];
  labels: string[];
  states: ExternalCanisterStateEnum[];
};

export type ExternalCanistersFiltersStorable = {
  name_prefix?: string;
  labels?: string[];
  canisters?: string[]; // format: {principal},{name}
  states?: ExternalCanisterStateEnum[];
};

const getDefaultFilters = (): ExternalCanistersFilters => ({
  name_prefix: '',
  labels: [],
  canisters: [],
  canister_items: [],
  states: [ExternalCanisterStateEnum.Active],
});

const parseFromStorableCanisterEntry = (entry: string): SelectItem<string> | null => {
  try {
    if (!entry.includes(',')) {
      throw new Error('Invalid entry format');
    }

    const parts = entry.split(',');
    const id = Principal.fromText(parts[0]);
    const name = parts.slice(1).join(',');

    return { value: id.toText(), text: name };
  } catch (e) {
    logger.error('Failed to parse canister entry', e);

    return null;
  }
};

const mapToStorableCanisterEntry = (canister: SelectItem<string>): string =>
  `${canister.value},${canister.text}`;

const buildFilters = (rawQuery: LocationQuery): ExternalCanistersFilters => {
  try {
    const defaultFilters = getDefaultFilters();
    const query = rawQuery as ExternalCanistersFiltersStorable;
    let states = query?.states ?? defaultFilters.states;
    if (!Array.isArray(states)) {
      states = [states];
    }

    let labels = query?.labels ?? defaultFilters.labels;
    if (!Array.isArray(labels)) {
      labels = [labels];
    }

    let canisters = query?.canisters ?? [];
    if (!Array.isArray(canisters)) {
      canisters = [canisters];
    }

    const canisterItems = canisters
      .map(parseFromStorableCanisterEntry)
      .filter(Boolean) as SelectItem<string>[];

    return {
      name_prefix: query?.name_prefix ?? defaultFilters.name_prefix,
      labels,
      states,
      canisters: canisterItems.map(entry => entry.value),
      canister_items: canisterItems,
    };
  } catch (e) {
    logger.error('Failed to parse filters from query', e);

    return getDefaultFilters();
  }
};

export const useExternalCanistersStates = (): ComputedRef<
  {
    key: ExternalCanisterStateEnum;
    text: string;
  }[]
> => {
  const i18n = useI18n();

  return computed(() => [
    { key: ExternalCanisterStateEnum.Active, text: i18n.t('terms.active') },
    { key: ExternalCanisterStateEnum.Archived, text: i18n.t('terms.archived') },
  ]);
};

export const useExternalCanistersFilters = () => {
  const router = useRouter();
  const filters = ref(buildFilters(router.currentRoute.value.query));

  return {
    fields: filters,
    save() {
      const selectedCanistersSet = new Set(filters.value.canisters);
      const filterByCanisters = filters.value.canister_items.filter(canister =>
        selectedCanistersSet.has(canister.value),
      );

      router.replace({
        query: {
          name_prefix: filters.value.name_prefix?.length ? filters.value.name_prefix : undefined,
          labels: filters.value.labels,
          states: filters.value.states,
          canisters: filterByCanisters.map(mapToStorableCanisterEntry),
        },
      });
    },
    reset() {
      filters.value = getDefaultFilters();
    },
  };
};

export const useExternalCanistersAutocomplete = () => {
  const station = useStationStore();

  const autocomplete = useAutocomplete(async term => {
    const result = await station.service.fetchExternalCanisterFilters({
      with_labels: false,
      with_name: term?.trim(),
    });

    return result.names?.[0] ?? [];
  });

  return autocomplete;
};
