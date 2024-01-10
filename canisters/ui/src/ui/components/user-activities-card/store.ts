import { defineStore } from 'pinia';

export interface StoreState {
  loading: boolean;
}

export const useStore = (sectionId: string = 'main') =>
  defineStore(`user-activities-card-${sectionId}`, {
    state: (): StoreState => {
      return {
        loading: false,
      };
    },
  })();
