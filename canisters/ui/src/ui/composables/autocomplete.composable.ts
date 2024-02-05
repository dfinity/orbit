import { Ref, ref } from 'vue';
import { logger, throttle } from '~/core';
import { UserGroup } from '~/generated/wallet/wallet.did';
import { useWalletStore } from '~/ui/stores/wallet';

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
