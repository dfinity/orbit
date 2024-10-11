import { describe, expect, it } from 'vitest';
import { useRouter } from 'vue-router';
import { useStationStore } from '~/stores/station.store';
import { RequestDomains, RequestStatusEnum } from '~/types/station.types';
import { setupComponent } from '../test.utils';
import {
  StorableFilters,
  useAvailableDomains,
  useRequestStatusItems,
  useSavedFilters,
} from './request.composable';

describe('Request Composables', () => {
  it('should not load domains the user is missing privileges', () => {
    const vm = setupComponent(() => {
      const station = useStationStore();
      station.privileges = [];

      return { availableDomains: useAvailableDomains() };
    });

    expect(vm.availableDomains.map(domain => domain.id)).not.toContainEqual(RequestDomains.Users);
  });

  it('should load domains if the user has required privileges', () => {
    const vm = setupComponent(() => {
      const station = useStationStore();
      station.privileges = [{ ListUsers: null }];

      return { availableDomains: useAvailableDomains() };
    });

    expect(vm.availableDomains.map(domain => domain.id)).toContainEqual(RequestDomains.Users);
  });

  it('should include all request statuses', () => {
    const vm = setupComponent(() => ({
      statuses: useRequestStatusItems(),
    }));

    expect(vm.statuses.map(status => status.key)).toEqual(Object.values(RequestStatusEnum));
  });

  it('should fill filters with router query params', () => {
    const vm = setupComponent(() => {
      const router = useRouter();

      router.currentRoute.value.query = {
        group_by: RequestDomains.System,
        created_from: '2020-01-01',
        created_to: '2021-02-01',
        expires_from: '2020-01-01',
        expires_to: '2021-02-01',
        statuses: [RequestStatusEnum.Completed],
      } as StorableFilters;

      return { filters: useSavedFilters() };
    });

    expect(vm.filters.groupBy).toEqual(RequestDomains.System);
    expect(vm.filters.created.from).toEqual(new Date('2020-01-01'));
    expect(vm.filters.created.to).toEqual(new Date('2021-02-01'));
    expect(vm.filters.expires.from).toEqual(new Date('2020-01-01'));
    expect(vm.filters.expires.to).toEqual(new Date('2021-02-01'));
    expect(vm.filters.statuses).toEqual([RequestStatusEnum.Completed]);
  });
});
