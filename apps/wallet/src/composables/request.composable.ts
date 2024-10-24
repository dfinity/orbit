import { Principal } from '@dfinity/principal';
import { ComputedRef, Ref, computed, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { useRouter } from 'vue-router';
import { DateRangeModel } from '~/components/inputs/DateRange.vue';
import { REQUEST_DIALOG_QUERY_PARAM } from '~/core/constants.core';
import { logger } from '~/core/logger.core';
import { ListRequestsOperationType, UUID } from '~/generated/station/station.did';
import { mapListRequestsOperationTypeToGroups } from '~/mappers/requests.mapper';
import { i18n } from '~/plugins/i18n.plugin';
import { useAppStore } from '~/stores/app.store';
import { Privilege } from '~/types/auth.types';
import { SelectItem } from '~/types/helper.types';
import { ListRequestsOperationTypeGroup } from '~/types/requests.types';
import { RequestDomains, RequestSpecifierEnum, RequestStatusEnum } from '~/types/station.types';
import { hasRequiredPrivilege } from '~/utils/auth.utils';
import { parseDate } from '~/utils/date.utils';

export type AvailableDomain = {
  id: RequestDomains;
  types: ListRequestsOperationType[];
};

export const useAvailableDomains = (): Ref<AvailableDomain[]> => {
  const domains: Ref<AvailableDomain[]> = ref([]);
  domains.value.push({
    id: RequestDomains.All,
    types: [],
  });

  if (hasRequiredPrivilege({ anyOf: [Privilege.ListAccounts] })) {
    domains.value.push({
      id: RequestDomains.Accounts,
      types: [{ AddAccount: null }, { EditAccount: null }],
    });

    domains.value.push({
      id: RequestDomains.Transfers,
      types: [{ Transfer: [] }],
    });
  }

  if (hasRequiredPrivilege({ anyOf: [Privilege.ListUsers] })) {
    domains.value.push({
      id: RequestDomains.Users,
      types: [{ AddUser: null }, { EditUser: null }],
    });
  }

  if (hasRequiredPrivilege({ anyOf: [Privilege.ListExternalCanisters] })) {
    domains.value.push({
      id: RequestDomains.ExternalCanisters,
      types: [
        { CreateExternalCanister: null },
        { FundExternalCanister: [] },
        { ConfigureExternalCanister: [] },
        { CallExternalCanister: [] },
        { ChangeExternalCanister: [] },
      ],
    });
  }

  if (hasRequiredPrivilege({ anyOf: [Privilege.ListAddressBookEntries] })) {
    domains.value.push({
      id: RequestDomains.AddressBook,
      types: [
        { AddAddressBookEntry: null },
        { EditAddressBookEntry: null },
        { RemoveAddressBookEntry: null },
      ],
    });
  }

  if (hasRequiredPrivilege({ anyOf: [Privilege.ListAssets] })) {
    domains.value.push({
      id: RequestDomains.Assets,
      types: [{ AddAsset: null }, { EditAsset: null }, { RemoveAsset: null }],
    });
  }

  domains.value.push({
    id: RequestDomains.System,
    types: [
      { EditPermission: null },
      { AddRequestPolicy: null },
      { EditRequestPolicy: null },
      { RemoveRequestPolicy: null },
      { SystemUpgrade: null },
      { AddUserGroup: null },
      { EditUserGroup: null },
      { RemoveUserGroup: null },
      { ManageSystemInfo: null },
    ],
  });

  return domains;
};

export type Filters = {
  groupBy: RequestDomains;
  created: DateRangeModel;
  expires: DateRangeModel;
  statuses: RequestStatusEnum[];
  canisterId?: Principal;
};

export type StorableFilters = {
  group_by?: string;
  created_from?: string;
  created_to?: string;
  expires_from?: string;
  expires_to?: string;
  statuses?: RequestStatusEnum[];
  canister_id?: string;
};

export type FilterUtils = {
  getQuery(filters: Filters): StorableFilters;
  getDefaultFilters(): Filters;
};

const getDefaultFilters = (): Filters => ({
  groupBy: RequestDomains.All,
  created: {
    from: new Date(new Date().setDate(new Date().getDate() - 30)),
    to: new Date(),
  },
  expires: {
    from: undefined,
    to: undefined,
  },
  statuses: [RequestStatusEnum.Created],
  canisterId: undefined,
});

export const useFilterUtils = (): FilterUtils => {
  const availableDomains = useAvailableDomains();

  return {
    getDefaultFilters: getDefaultFilters,
    getQuery: (filters: Filters): StorableFilters => {
      const { groupBy, created, expires, statuses, canisterId } = filters;
      const storableFilters: StorableFilters = {
        created_from: created.from?.toISOString(),
        created_to: created.to?.toISOString(),
        expires_from: expires.from?.toISOString(),
        expires_to: expires.to?.toISOString(),
        group_by: availableDomains.value.find(domain => domain.id === groupBy)?.id,
        statuses: statuses,
        canister_id: canisterId ? canisterId.toText() : undefined,
      };

      return storableFilters;
    },
  };
};

export const useSavedFilters = (): Ref<Filters> => {
  const defaultFilters = useFilterUtils().getDefaultFilters();
  const app = useAppStore();
  const router = useRouter();

  try {
    const query = router.currentRoute.value.query as StorableFilters;
    const createdDt: DateRangeModel = {
      from: query?.created_from ? parseDate(query.created_from) : defaultFilters.created.from,
      to: query?.created_to ? parseDate(query.created_to) : defaultFilters.created.to,
    };

    createdDt.from = createdDt.from! > createdDt.to! ? createdDt.to : createdDt.from;
    createdDt.to = createdDt.to! < createdDt.from! ? createdDt.from : createdDt.to;

    const expiresDt: DateRangeModel = {
      from: query?.expires_from ? parseDate(query.expires_from) : defaultFilters.expires.from,
      to: query?.expires_to ? parseDate(query.expires_to) : defaultFilters.expires.to,
    };

    expiresDt.from = expiresDt.from! > expiresDt.to! ? expiresDt.to : expiresDt.from;
    expiresDt.to = expiresDt.to! < expiresDt.from! ? expiresDt.from : expiresDt.to;

    let statuses = query?.statuses ?? defaultFilters.statuses;
    if (!Array.isArray(statuses)) {
      statuses = [statuses];
    }

    const queryParamGroupBy =
      query?.group_by && Array.isArray(query.group_by) ? query.group_by[0] : query.group_by;

    const groupBy =
      queryParamGroupBy &&
      Object.values(RequestDomains).includes(queryParamGroupBy as RequestDomains)
        ? queryParamGroupBy
        : defaultFilters.groupBy;

    const queryParamCanisterId =
      query?.canister_id && Array.isArray(query.canister_id) && query.canister_id.length > 0
        ? query.canister_id[0]
        : query.canister_id;

    const canisterId =
      queryParamCanisterId && queryParamCanisterId?.trim().length
        ? Principal.fromText(queryParamCanisterId?.trim())
        : undefined;

    return ref({
      groupBy,
      created: createdDt,
      expires: expiresDt,
      statuses: Object.values(RequestStatusEnum).filter(status => statuses.includes(status)),
      canisterId,
    }) as Ref<Filters>;
  } catch (e) {
    logger.error('Failed to parse filters from query', e);

    app.sendNotification({
      type: 'error',
      message: i18n.global.t('app.params_parse_error'),
    });

    return ref(defaultFilters) as Ref<Filters>;
  }
};

export type RequestStatusSelectItem = { key: RequestStatusEnum; text: string };

export const useRequestStatusItems = (): ComputedRef<RequestStatusSelectItem[]> =>
  computed(() =>
    Object.values(RequestStatusEnum).map(status => ({
      key: status,
      text: i18n.global.t(`requests.status.${status.toLowerCase()}`),
    })),
  );

export const useAvailableORequestSpecifiers = (): SelectItem[] => {
  const i18n = useI18n();
  const items: SelectItem<string>[] = [];

  for (const specifier in RequestSpecifierEnum) {
    items.push({
      value: specifier,
      text: i18n.t(`request_policies.specifier.${specifier.toLowerCase()}`),
    });
  }

  items.sort((a, b) => a.text.localeCompare(b.text));

  return items;
};

export const useRequestOverlay = (): {
  open: (requestId: UUID) => void;
  close: () => void;
  replaceQueryId: (id: UUID | undefined) => void;
} => {
  const router = useRouter();

  const open = (requestId: UUID): void => {
    router.push({ query: { [REQUEST_DIALOG_QUERY_PARAM]: requestId } });
  };

  const replaceQueryId = (id: UUID | undefined): void => {
    const query = Object.assign({}, router.currentRoute.value.query);
    if (id) {
      query[REQUEST_DIALOG_QUERY_PARAM] = id;
    } else {
      delete query[REQUEST_DIALOG_QUERY_PARAM];
    }

    router.replace({ query });
  };

  const close = (): void => {
    // Delay to allow the dialog to close before removing the query param
    setTimeout(() => {
      replaceQueryId(undefined);
    }, 100);
  };

  return { open, close, replaceQueryId };
};

export interface DownloadItem {
  downloading: boolean;
  group: ListRequestsOperationTypeGroup;
  filterBy: {
    types: ListRequestsOperationType[];
    created: DateRangeModel;
    expires: DateRangeModel;
    statuses: RequestStatusEnum[];
  };
}

export const useDownloadItems = (
  filters: Ref<Filters>,
  domains: Ref<AvailableDomain[]>,
): Ref<DownloadItem[]> => {
  const downloads: Ref<DownloadItem[]> = ref([]);

  const createDownloadList = (): void => {
    const items: DownloadItem[] = [];
    if (!domains.value.length) {
      downloads.value = [];
      return;
    }
    const types = domains.value.find(domain => domain.id === filters.value.groupBy)?.types ?? [];
    if (!types.length) {
      types.push(...domains.value.map(d => d.types).flat());
    }

    const downloadGroups = mapListRequestsOperationTypeToGroups(types);
    for (const [group, types] of downloadGroups) {
      items.push({
        downloading: false,
        group,
        filterBy: {
          types,
          created: filters.value.created,
          expires: filters.value.expires,
          statuses: filters.value.statuses,
        },
      });
    }

    downloads.value = items;
  };

  watch(
    () => filters.value,
    () => createDownloadList(),
    { deep: true, immediate: true },
  );

  return downloads;
};
