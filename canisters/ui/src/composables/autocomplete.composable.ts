import { Ref, ref } from 'vue';
import { logger } from '~/core/logger.core';
import { UserGroup } from '~/generated/wallet/wallet.did';
import { useWalletStore } from '~/stores/wallet.store';
import { throttle } from '~/utils/helper.utils';

export const useAutocomplete = <T>(fetchCall: (search: string) => Promise<T[]>) => {
  const search = ref<string>('');
  const loading = ref<boolean>(false);
  const results: Ref<T[]> = ref([]);

  const triggerSearch = throttle(async () => {
    try {
      loading.value = true;
      const fetchResults = await fetchCall(search.value);

      results.value = fetchResults;
    } catch (err) {
      logger.error(`Failed to search`, { err });

      results.value = [];
    } finally {
      loading.value = false;
    }
  }, 500);

  const searchItems = async (searchTerm?: string): Promise<void> => {
    search.value = searchTerm || '';

    triggerSearch();
  };

  return {
    search,
    loading,
    results,
    searchItems,
  };
};

export const useUserGroupsAutocomplete = () => {
  const wallet = useWalletStore();

  const autocomplete = useAutocomplete<UserGroup>(async term => {
    const groups = await wallet.service.listUserGroups({
      searchTerm: term.trim().length > 0 ? term.trim() : undefined,
      limit: 100,
      offset: 0,
    });

    return groups.user_groups;
  });

  return autocomplete;
};

export const useUsersAutocomplete = () => {
  const wallet = useWalletStore();

  const autocomplete = useAutocomplete(async term => {
    const users = await wallet.service.listUsers({
      searchTerm: term.trim().length > 0 ? term.trim() : undefined,
      limit: 100,
      offset: 0,
    });

    return users.users;
  });

  return autocomplete;
};

export const useAccountsAutocomplete = () => {
  const wallet = useWalletStore();

  const autocomplete = useAutocomplete(async term => {
    const accounts = await wallet.service.listAccounts({
      searchTerm: term.trim().length > 0 ? term.trim() : undefined,
      limit: 100,
      offset: 0,
    });

    return accounts.accounts;
  });

  return autocomplete;
};


export const useAddressBookAutocomplete = () => {
  const wallet = useWalletStore();

  const autocomplete = useAutocomplete(async term => {
    const results = await wallet.service.listAddressBook({
      addressOwner: term.trim().length > 0 ? term.trim() : undefined,
      limit: 100,
      offset: 0,
    });

    return results.address_book_entries;
  });

  return autocomplete;
};
