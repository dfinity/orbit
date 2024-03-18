import { ComputedRef, Ref, computed, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { useRouter } from 'vue-router';
import { DateRangeModel } from '~/components/inputs/DateRange.vue';
import { PROPOSAL_DIALOG_QUERY_PARAM } from '~/core/constants.core';
import { logger } from '~/core/logger.core';
import { ListProposalsOperationType, UUID } from '~/generated/wallet/wallet.did';
import { mapListProposalsOperationTypeToGroups } from '~/mappers/proposals.mapper';
import { i18n } from '~/plugins/i18n.plugin';
import { useAppStore } from '~/stores/app.store';
import { Privilege } from '~/types/auth.types';
import { SelectItem } from '~/types/helper.types';
import { ListProposalsOperationTypeGroup } from '~/types/proposals.types';
import { ProposalDomains, ProposalSpecifierEnum, ProposalStatusEnum } from '~/types/wallet.types';
import { hasRequiredPrivilege } from '~/utils/auth.utils';
import { parseDate } from '~/utils/date.utils';

export type AvailableDomain = {
  id: ProposalDomains;
  types: ListProposalsOperationType[];
};

export const useAvailableDomains = (): Ref<AvailableDomain[]> => {
  const domains: AvailableDomain[] = [];
  domains.push({
    id: ProposalDomains.All,
    types: [],
  });

  if (hasRequiredPrivilege({ anyOf: [Privilege.ListAccounts] })) {
    domains.push({
      id: ProposalDomains.Accounts,
      types: [{ AddAccount: null }, { EditAccount: null }],
    });

    domains.push({
      id: ProposalDomains.Transfers,
      types: [{ Transfer: [] }],
    });
  }

  if (hasRequiredPrivilege({ anyOf: [Privilege.ListUsers] })) {
    domains.push({
      id: ProposalDomains.Users,
      types: [{ AddUser: null }, { EditUser: null }],
    });
  }

  if (hasRequiredPrivilege({ anyOf: [Privilege.ListAddressBookEntries] })) {
    domains.push({
      id: ProposalDomains.AddressBook,
      types: [
        { AddAddressBookEntry: null },
        { EditAddressBookEntry: null },
        { RemoveAddressBookEntry: null },
      ],
    });
  }

  domains.push({
    id: ProposalDomains.System,
    types: [
      { EditAccessPolicy: null },
      { AddProposalPolicy: null },
      { EditProposalPolicy: null },
      { RemoveProposalPolicy: null },
      { ChangeCanister: null },
      { AddUserGroup: null },
      { EditUserGroup: null },
      { RemoveUserGroup: null },
    ],
  });

  return ref(domains);
};

export type Filters = {
  groupBy: number;
  created: DateRangeModel;
  expires: DateRangeModel;
  statuses: ProposalStatusEnum[];
};

export type StorableFilters = {
  group_by?: string;
  created_from?: string;
  created_to?: string;
  expires_from?: string;
  expires_to?: string;
  statuses?: ProposalStatusEnum[];
};

export type FilterUtils = {
  getQuery(filters: Filters): StorableFilters;
  getDefaultFilters(): Filters;
};

const getDefaultFilters = (): Filters => ({
  groupBy: 0,
  created: {
    from: new Date(new Date().setDate(new Date().getDate() - 30)),
    to: new Date(),
  },
  expires: {
    from: undefined,
    to: undefined,
  },
  statuses: [ProposalStatusEnum.Created],
});

export const useFilterUtils = (): FilterUtils => {
  const availableDomains = useAvailableDomains();

  return {
    getDefaultFilters: getDefaultFilters,
    getQuery: (filters: Filters): StorableFilters => {
      const { groupBy, created, expires, statuses } = filters;
      const storableFilters: StorableFilters = {
        created_from: created.from?.toISOString(),
        created_to: created.to?.toISOString(),
        expires_from: expires.from?.toISOString(),
        expires_to: expires.to?.toISOString(),
        group_by: availableDomains.value.find((_, idx) => idx === groupBy)?.id,
        statuses: statuses,
      };

      return storableFilters;
    },
  };
};

export const useSavedFilters = (availableDomains: AvailableDomain[]): Ref<Filters> => {
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

    let groupBy = defaultFilters.groupBy;
    if (query?.group_by && availableDomains.findIndex(group => group.id === query.group_by) > -1) {
      groupBy = availableDomains.findIndex(group => group.id === query.group_by);
    }

    return ref({
      groupBy,
      created: createdDt,
      expires: expiresDt,
      statuses: Object.values(ProposalStatusEnum).filter(status => statuses.includes(status)),
    });
  } catch (e) {
    logger.error('Failed to parse filters from query', e);

    app.sendNotification({
      type: 'error',
      message: i18n.global.t('app.params_parse_error'),
    });

    return ref(defaultFilters);
  }
};

export type ProposalStatusSelectItem = { key: ProposalStatusEnum; text: string };

export const useProposalStatusItems = (): ComputedRef<ProposalStatusSelectItem[]> =>
  computed(() =>
    Object.values(ProposalStatusEnum).map(status => ({
      key: status,
      text: i18n.global.t(`proposals.status.${status.toLowerCase()}`),
    })),
  );

export const useAvailableOProposalSpecifiers = (): SelectItem[] => {
  const i18n = useI18n();
  const items: SelectItem<string>[] = [];

  for (const specifier in ProposalSpecifierEnum) {
    items.push({
      value: specifier,
      text: i18n.t(`proposal_policies.specifier.${specifier.toLowerCase()}`),
    });
  }

  items.sort((a, b) => a.text.localeCompare(b.text));

  return items;
};

export const useProposalOverlay = (): {
  open: (proposalId: UUID) => void;
  close: () => void;
  replaceQueryId: (id: UUID | undefined) => void;
} => {
  const router = useRouter();

  const open = (proposalId: UUID): void => {
    router.push({ query: { [PROPOSAL_DIALOG_QUERY_PARAM]: proposalId } });
  };

  const replaceQueryId = (id: UUID | undefined): void => {
    const query = Object.assign({}, router.currentRoute.value.query);
    if (id) {
      query[PROPOSAL_DIALOG_QUERY_PARAM] = id;
    } else {
      delete query[PROPOSAL_DIALOG_QUERY_PARAM];
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
  group: ListProposalsOperationTypeGroup;
  filterBy: {
    types: ListProposalsOperationType[];
    created: DateRangeModel;
    expires: DateRangeModel;
    statuses: ProposalStatusEnum[];
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
    const types = domains.value.find((_, idx) => idx === filters.value.groupBy)?.types ?? [];
    if (!types.length) {
      types.push(...domains.value.map(d => d.types).flat());
    }

    const downloadGroups = mapListProposalsOperationTypeToGroups(types);
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
