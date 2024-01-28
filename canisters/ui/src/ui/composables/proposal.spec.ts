import { describe, expect, it } from 'vitest';
import { useWalletStore } from '~/ui/stores/wallet';
import { setupComponent } from '../test.utils';
import {
  StorableFilters,
  useAvailableDomains,
  useProposalStatusItems,
  useSavedFilters,
} from './proposal.composable';
import { ProposalDomains, ProposalStatusEnum } from '~/types';
import { useRouter } from 'vue-router';

describe('Proposal Composables', () => {
  it('should not load domains the user is missing privileges', () => {
    const vm = setupComponent(() => {
      const wallet = useWalletStore();
      wallet.privileges = [];

      return { availableDomains: useAvailableDomains() };
    });

    expect(vm.availableDomains.map(domain => domain.id)).not.toContainEqual(ProposalDomains.Users);
  });

  it('should load domains if the user has required privileges', () => {
    const vm = setupComponent(() => {
      const wallet = useWalletStore();
      wallet.privileges = [{ ListUsers: null }];

      return { availableDomains: useAvailableDomains() };
    });

    expect(vm.availableDomains.map(domain => domain.id)).toContainEqual(ProposalDomains.Users);
  });

  it('should include all proposal statuses', () => {
    const vm = setupComponent(() => ({
      statuses: useProposalStatusItems(),
    }));

    expect(vm.statuses.map(status => status.key)).toEqual(Object.values(ProposalStatusEnum));
  });

  it('should fill filters with router query params', () => {
    const vm = setupComponent(() => {
      const router = useRouter();

      router.currentRoute.value.query = {
        group_by: ProposalDomains.System,
        created_from: '2020-01-01',
        created_to: '2021-02-01',
        expires_from: '2020-01-01',
        expires_to: '2021-02-01',
        statuses: [ProposalStatusEnum.Completed],
      } as StorableFilters;

      return { filters: useSavedFilters() };
    });

    expect(vm.filters.groupBy).toEqual(1);
    expect(vm.filters.created.from).toEqual(new Date('2020-01-01'));
    expect(vm.filters.created.to).toEqual(new Date('2021-02-01'));
    expect(vm.filters.expires.from).toEqual(new Date('2020-01-01'));
    expect(vm.filters.expires.to).toEqual(new Date('2021-02-01'));
    expect(vm.filters.statuses).toEqual([ProposalStatusEnum.Completed]);
  });
});
