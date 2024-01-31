import { ComputedRef, Ref, computed, ref } from 'vue';
import { useRouter } from 'vue-router';
import { logger, parseDate } from '~/core';
import { ListProposalsOperationType } from '~/generated/wallet/wallet.did';
import { Privilege, ProposalDomains, ProposalStatusEnum } from '~/types';
import { DateRangeModel } from '~/ui/components/inputs/DateRange.vue';
import { i18n } from '~/ui/modules';
import { useAppStore } from '~/ui/stores/app';
import { hasRequiredPrivilege } from '~/ui/utils/auth';

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
      { AddAccessPolicy: null },
      { EditAccessPolicy: null },
      { RemoveAccessPolicy: null },
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

export const useSavedFilters = (): Ref<Filters> => {
  const defaultFilters = useFilterUtils().getDefaultFilters();
  const availableDomains = useAvailableDomains();
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
    if (
      query?.group_by &&
      availableDomains.value.findIndex(group => group.id === query.group_by) > -1
    ) {
      groupBy = availableDomains.value.findIndex(group => group.id === query.group_by);
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
