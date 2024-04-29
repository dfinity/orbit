import { flushPromises } from '@vue/test-utils';
import { describe, expect, it, vi } from 'vitest';
import { icAgent } from '~/core/ic-agent.core';
import { ListRequestsResult } from '~/generated/station/station.did';
import { serviceManager } from '~/plugins/services.plugin';
import { StationService } from '~/services/station.service';
import { mount } from '~/test.utils';
import { ExtractOk } from '~/types/helper.types';
import RequestsPage from './RequestsPage.vue';

vi.mock('~/services/station.service', () => {
  const mock: Partial<StationService> = {
    withStationId: vi.fn().mockReturnThis(),
    listRequests: vi.fn().mockReturnThis(),
  };

  return {
    StationService: vi.fn(() => mock),
  };
});

const mockedStationService = new StationService(icAgent.get());
serviceManager.services.station = mockedStationService;

vi.spyOn(mockedStationService, 'listRequests').mockReturnValue(
  Promise.resolve({
    requests: [],
    additional_info: [],
    privileges: [],
    next_offset: [],
    total: BigInt(0),
  } as ExtractOk<ListRequestsResult>),
);

describe('RequestsPage', () => {
  it('renders properly', () => {
    const wrapper = mount(RequestsPage);

    expect(wrapper.exists()).toBe(true);
  });

  it('renders with empty list of requests', async () => {
    const wrapper = mount(RequestsPage);
    const pageBody = wrapper.find('[data-test-id="page-body"]');

    expect(pageBody.exists()).toBe(true);

    await wrapper.vm.$nextTick();

    expect(pageBody.find('[data-test-id="loading"]').exists()).toBe(true);

    await flushPromises();

    expect(pageBody.find('[data-test-id="requests-empty-list"]').exists()).toBe(true);
  });

  it('renders with request list', async () => {
    vi.spyOn(mockedStationService, 'listRequests').mockReturnValue(
      Promise.resolve({
        requests: [
          {
            id: '1',
            created_at: new Date().toISOString(),
            status: { Approved: null },
            operation: {
              AddUserGroup: {
                input: {
                  name: 'finance',
                },
              },
            },
          },
        ],
        next_offset: [BigInt(1)],
        total: BigInt(2),
      } as ExtractOk<ListRequestsResult>),
    );

    const wrapper = mount(RequestsPage);
    const pageBody = wrapper.find('[data-test-id="page-body"]');

    expect(pageBody.exists()).toBe(true);

    await wrapper.vm.$nextTick();

    expect(pageBody.find('[data-test-id="loading"]').exists()).toBe(true);

    await flushPromises();

    expect(pageBody.find('[data-test-id="loading"]').exists()).toBe(false);
    expect(pageBody.find('[data-test-id="requests-empty-list"]').exists()).toBe(false);
  });
});
