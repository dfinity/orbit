import { defineStore } from 'pinia';
import { UUID } from '~/generated/wallet/wallet.did';
import { useWalletStore } from '~/ui/stores/wallet';

export type AutocompleteSet = {
  [key: string]: string;
};

export interface AutocompleteStoreState {
  users: {
    loading: boolean;
    items: AutocompleteSet;
  };
  userGroups: {
    loading: boolean;
    items: AutocompleteSet;
  };
}

const initialStoreState = (): AutocompleteStoreState => {
  return {
    users: {
      loading: false,
      items: {},
    },
    userGroups: {
      loading: false,
      items: {},
    },
  };
};

export const useAutocompleteStore = defineStore('autocomplete', {
  state: (): AutocompleteStoreState => initialStoreState(),
  getters: {
    user(state): (id: UUID) => string {
      return (id: UUID): string => {
        return state.users.items?.[id] || '-';
      };
    },
    userGroup(state): (id: UUID) => string {
      return (id: UUID): string => {
        return state.userGroups.items?.[id] || '-';
      };
    },
  },
  actions: {
    reset(): void {
      const initialState = initialStoreState();

      this.users = initialState.users;
      this.userGroups = initialState.userGroups;
    },
    async fetchUserGroupsByIds(ids: string[]): Promise<void> {
      try {
        this.userGroups.loading = true;
        const missingIds = ids.filter(id => !this.userGroups.items[id]);
        const wallet = useWalletStore();

        const groups = await wallet.configuration.details.user_groups.filter(group => {
          return missingIds.includes(group.id);
        });

        groups.forEach(group => {
          this.userGroups.items[group.id] = group.name;
        });
      } finally {
        this.userGroups.loading = false;
      }
    },
    async fetchUsersByIds(_ids: string[]): Promise<void> {
      try {
        this.users.loading = true;
        const wallet = useWalletStore();

        const { users } = await wallet.service.listUsers();

        users.forEach(user => {
          this.users.items[user.id] = user.name?.[0] || '-';
        });
      } finally {
        this.users.loading = false;
      }
    },
  },
});
