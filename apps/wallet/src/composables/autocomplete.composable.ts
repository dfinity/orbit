import { Ref, ref } from 'vue';
import { logger } from '~/core/logger.core';
import { UserGroup } from '~/generated/station/station.did';
import { useStationStore } from '~/stores/station.store';
import { debounce } from '~/utils/helper.utils';

export const useAutocomplete = <T>(fetchCall: (search: string) => Promise<T[]>) => {
  const search = ref<string>('');
  const loading = ref<boolean>(false);
  const results: Ref<T[]> = ref([]);

  const triggerSearch = debounce(async () => {
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

    return triggerSearch();
  };

  return {
    search,
    loading,
    results,
    searchItems,
  };
};

export const useUserGroupsAutocomplete = () => {
  const station = useStationStore();

  const autocomplete = useAutocomplete<UserGroup>(async term => {
    const groups = await station.service.listUserGroups({
      searchTerm: term.trim().length > 0 ? term.trim() : undefined,
      limit: 100,
      offset: 0,
    });

    return groups.user_groups;
  });

  return autocomplete;
};

export const useUsersAutocomplete = () => {
  const station = useStationStore();

  const autocomplete = useAutocomplete(async term => {
    const users = await station.service.listUsers({
      searchTerm: term.trim().length > 0 ? term.trim() : undefined,
      limit: 100,
      offset: 0,
    });

    return users.users;
  });

  return autocomplete;
};

export const useAccountsAutocomplete = () => {
  const station = useStationStore();

  const autocomplete = useAutocomplete(async term => {
    const accounts = await station.service.listAccounts({
      searchTerm: term.trim().length > 0 ? term.trim() : undefined,
      limit: 100,
      offset: 0,
    });

    return accounts.accounts;
  });

  return autocomplete;
};

export const useAddressBookAutocomplete = () => {
  const station = useStationStore();

  const autocomplete = useAutocomplete(async term => {
    const results = await station.service.listAddressBook({
      search_term: term.trim().length > 0 ? term.trim() : undefined,
      limit: 10,
      offset: 0,
    });

    return results.address_book_entries;
  });

  return autocomplete;
};

export const useAssetAutocomplete = () => {
  const station = useStationStore();

  const autocomplete = useAutocomplete(async () => {
    const results = await station.service.listAssets({
      limit: 100,
      offset: 0,
    });

    return results.assets;
  });

  return autocomplete;
};
