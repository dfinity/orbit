import { Principal } from '@dfinity/principal';
import { ComputedRef, computed, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { LocationQuery, useRouter } from 'vue-router';
import { CanisterWizardModel } from '~/components/external-canisters/wizard/wizard.types';
import { useAutocomplete } from '~/composables/autocomplete.composable';
import { icAgent } from '~/core/ic-agent.core';
import logger from '~/core/logger.core';
import { ApiError } from '~/generated/control-panel/control_panel.did';
import {
  CanisterStatusResult,
  GetExternalCanisterResult,
  UUID,
} from '~/generated/station/station.did';
import { mapExternalCanisterStateVariantToEnum } from '~/mappers/external-canister.mapper';
import { useStationStore } from '~/stores/station.store';
import { ExtractOk, SelectItem } from '~/types/helper.types';
import { ExternalCanisterStateEnum } from '~/types/station.types';
import { compactArray, fetchCanisterModuleHash, parseLocationQuery } from '~/utils/helper.utils';

export type ExternalCanistersFilters = {
  name_prefix: string;
  canisters: string[];
  canister_items: SelectItem<string>[];
  labels: string[];
  states: ExternalCanisterStateEnum[];
  sort_by: string;
};

export type ExternalCanistersFiltersStorable = {
  name_prefix?: string;
  labels?: string[];
  canisters?: string[]; // format: {principal},{name}
  states?: ExternalCanisterStateEnum[];
  sort_by?: string;
};

const getDefaultFilters = (): ExternalCanistersFilters => ({
  name_prefix: '',
  labels: [],
  canisters: [],
  canister_items: [],
  states: [ExternalCanisterStateEnum.Active],
  sort_by: 'name_asc',
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
    const query = parseLocationQuery(rawQuery);
    const defaultFilters = getDefaultFilters();
    const storaredFields: ExternalCanistersFiltersStorable = {};

    if (query.labels?.length) {
      storaredFields.labels = compactArray(query.labels);
    }

    if (query.states?.length) {
      storaredFields.states = compactArray<string, ExternalCanisterStateEnum>(query.states, {
        include: new Set(Object.values(ExternalCanisterStateEnum)),
      });
    }

    if (query.name_prefix?.length) {
      storaredFields.name_prefix = query.name_prefix[0];
    }

    const canisterItems = compactArray(
      (query?.canisters ?? []).map(parseFromStorableCanisterEntry),
    );

    let sort_by = defaultFilters.sort_by;
    switch (query?.sort_by?.[0]) {
      case 'name_desc':
        sort_by = 'name_desc';
        break;
      case 'name_asc':
      default:
        sort_by = 'name_asc';
        break;
    }

    return {
      name_prefix: storaredFields.name_prefix ?? defaultFilters.name_prefix,
      labels: storaredFields.labels ?? defaultFilters.labels,
      states: storaredFields.states ?? defaultFilters.states,
      canisters: canisterItems.map(entry => entry.value),
      canister_items: canisterItems,
      sort_by,
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

  return computed(() => {
    return Object.values(ExternalCanisterStateEnum).map(key => ({
      key,
      text: i18n.t(`terms.${key.toLowerCase()}`),
    }));
  });
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
          sort_by: filters.value.sort_by,
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

export const useDefaultExternalCanisterSetupWizardModel = ({
  prefilledUserIds,
}: {
  prefilledUserIds?: UUID[];
} = {}): CanisterWizardModel => {
  return {
    configuration: {
      name: '',
      description: '',
      labels: [],
      state: ExternalCanisterStateEnum.Active,
    },
    permission: {
      read: {
        auth_scope: { Restricted: null },
        user_groups: [],
        users: prefilledUserIds ? prefilledUserIds : [],
      },
      change: {
        auth_scope: { Restricted: null },
        user_groups: [],
        users: prefilledUserIds ? prefilledUserIds : [],
      },
    },
    approvalPolicy: {
      change: [],
    },
  };
};

export const useLoadExternaLCanisterSetupWizardModel = async (
  canisterId: Principal,
): Promise<CanisterWizardModel> => {
  const station = useStationStore();
  const { canister } = await station.service.getExternalCanisterByCanisterId(canisterId, true);

  return {
    configuration: {
      id: canister.id,
      canisterId: canister.canister_id,
      name: canister.name,
      description: canister.description?.[0] ? canister.description[0] : undefined,
      labels: canister.labels,
      state: mapExternalCanisterStateVariantToEnum(canister.state),
      createdAt: canister.created_at,
      modifiedAt: canister.modified_at?.[0] ?? '',
    },
    permission: {
      read: canister.permissions.read,
      change: canister.permissions.change,
    },
    approvalPolicy: {
      change: canister.request_policies.change,
    },
  };
};

export const useLoadExternalCanister = (opts: {
  canisterId: Principal;
  verifiedCall?: boolean;
}) => {
  const verifiedCall = opts?.verifiedCall ?? false;
  const canisterId = opts.canisterId;
  const station = useStationStore();

  const fetchExternalCanister = async (): Promise<
    Partial<ExtractOk<GetExternalCanisterResult>>
  > => {
    try {
      const result = await station.service.getExternalCanisterByCanisterId(
        canisterId,
        verifiedCall,
      );

      return result;
    } catch (err) {
      const error = err as ApiError;

      if (error?.code && error.code === 'INVALID_EXTERNAL_CANISTER') {
        return {
          canister: undefined,
          privileges: undefined,
        };
      }

      logger.error('Failed to load external canister', error);

      throw err;
    }
  };

  return fetchExternalCanister;
};

export const useLoadExternalCanisterModuleHash = async (
  canisterId: Principal,
): Promise<string | null> => {
  try {
    return await fetchCanisterModuleHash(icAgent.get(), canisterId);
  } catch (err) {
    logger.error('Failed to load canister module hash', err);
  }

  return null;
};

export const useLoadExternalCanisterStatus = async (
  canisterId: Principal,
): Promise<ExtractOk<CanisterStatusResult>> => {
  try {
    const station = useStationStore();

    return await station.service.getExternalCanisterStatus(canisterId);
  } catch (err) {
    logger.error('Failed to load canister status', err);

    throw err;
  }
};
