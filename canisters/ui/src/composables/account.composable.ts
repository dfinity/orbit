import { Ref, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { useRouter } from 'vue-router';
import { AccountSetupWizardModel } from '~/components/accounts/wizard/AccountSetupWizard.vue';
import { DateRangeModel } from '~/components/inputs/DateRange.vue';
import logger from '~/core/logger.core';
import { UUID } from '~/generated/wallet/wallet.did';
import { useAppStore } from '~/stores/app.store';
import { useWalletStore } from '~/stores/wallet.store';
import { BlockchainStandard, BlockchainType, TokenSymbol } from '~/types/chain.types';
import { parseDate } from '~/utils/date.utils';

export type Filters = {
  created: DateRangeModel;
};

export type StorableFilters = {
  created_from?: string;
  created_to?: string;
};

export type FilterUtils = {
  getQuery(filters: Filters): StorableFilters;
  getDefaultFilters(): Filters;
};

const getDefaultFilters = (): Filters => ({
  created: {
    from: new Date(new Date().setDate(new Date().getDate() - 7)),
    to: new Date(),
  },
});

export const useFilterUtils = (): FilterUtils => {
  return {
    getDefaultFilters: getDefaultFilters,
    getQuery: (filters: Filters): StorableFilters => {
      const { created } = filters;
      const storableFilters: StorableFilters = {
        created_from: created.from?.toISOString(),
        created_to: created.to?.toISOString(),
      };

      return storableFilters;
    },
  };
};

export const useSavedFilters = (): Ref<Filters> => {
  const defaultFilters = useFilterUtils().getDefaultFilters();
  const app = useAppStore();
  const router = useRouter();
  const i18n = useI18n();

  try {
    const query = router.currentRoute.value.query as StorableFilters;
    const createdDt: DateRangeModel = {
      from: query?.created_from ? parseDate(query.created_from) : defaultFilters.created.from,
      to: query?.created_to ? parseDate(query.created_to) : defaultFilters.created.to,
    };

    createdDt.from = createdDt.from! > createdDt.to! ? createdDt.to : createdDt.from;
    createdDt.to = createdDt.to! < createdDt.from! ? createdDt.from : createdDt.to;

    return ref({
      created: createdDt,
    });
  } catch (e) {
    logger.error('Failed to parse filters from query', e);

    app.sendNotification({
      type: 'error',
      message: i18n.t('app.params_parse_error'),
    });

    return ref(defaultFilters);
  }
};

export const useDefaultAccountSetupWizardModel = ({
  prefilledUserIds,
}: {
  prefilledUserIds?: UUID[];
} = {}): AccountSetupWizardModel => {
  return {
    configuration: {
      blockchain: BlockchainType.InternetComputer,
      standard: BlockchainStandard.Native,
      symbol: TokenSymbol.ICP,
    },
    access_policy: {
      read: {
        auth_scope: { Restricted: null },
        user_groups: [],
        users: prefilledUserIds ? prefilledUserIds : [],
      },
      configuration: {
        auth_scope: { Restricted: null },
        user_groups: [],
        users: prefilledUserIds ? prefilledUserIds : [],
      },
      transfer: {
        auth_scope: { Restricted: null },
        user_groups: [],
        users: prefilledUserIds ? prefilledUserIds : [],
      },
    },
    approval_policy: {},
  };
};

export const useLoadAccountSetupWizardModel = async (
  accountId: UUID,
): Promise<AccountSetupWizardModel> => {
  const wallet = useWalletStore();

  // load the individual account details and access policies in parallel
  const [account, read, configuration, transfer] = await Promise.all([
    wallet.service.getAccount({ account_id: accountId }, true).then(({ account }) => account),
    wallet.service
      .getAccessPolicy(
        {
          resource: { Account: { Read: { Id: accountId } } },
        },
        true,
      )
      .then(({ policy }) => policy.allow),
    wallet.service
      .getAccessPolicy(
        {
          resource: { Account: { Update: { Id: accountId } } },
        },
        true,
      )
      .then(({ policy }) => policy.allow),
    await wallet.service
      .getAccessPolicy(
        {
          resource: { Account: { Transfer: { Id: accountId } } },
        },
        true,
      )
      .then(({ policy }) => policy.allow),
  ]);

  return {
    configuration: {
      id: account.id,
      name: account.name,
      blockchain: account.blockchain,
      lastModified: account.last_modification_timestamp,
      standard: account.standard,
      symbol: account.symbol,
    },
    access_policy: {
      read,
      configuration,
      transfer,
    },
    approval_policy: {
      configurationCriteria: account.update_approval_policy?.[0],
      transferCriteria: account.transfer_approval_policy?.[0],
    },
  };
};
