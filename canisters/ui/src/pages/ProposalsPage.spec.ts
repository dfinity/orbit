import { flushPromises } from '@vue/test-utils';
import { describe, expect, it, vi } from 'vitest';
import { ListProposalsResult } from '~/generated/wallet/wallet.did';
import { serviceManager } from '~/modules/services.module';
import { WalletService } from '~/services/wallet.service';
import { mount } from '~/test.utils';
import { ExtractOk } from '~/types/utils.types';
import ProposalsPage from './ProposalsPage.vue';

vi.mock('~/services/WalletService', () => {
  const mock: Partial<WalletService> = {
    withWalletId: vi.fn().mockReturnThis(),
    listProposals: vi.fn().mockReturnThis(),
  };

  return {
    WalletService: vi.fn(() => mock),
  };
});

const mockedWalletService = new WalletService();
serviceManager.services.wallet = mockedWalletService;

vi.spyOn(mockedWalletService, 'listProposals').mockReturnValue(
  Promise.resolve({
    proposals: [],
    next_offset: [],
    total: BigInt(0),
  } as ExtractOk<ListProposalsResult>),
);

describe('ProposalsPage', () => {
  it('renders properly', () => {
    const wrapper = mount(ProposalsPage);

    expect(wrapper.exists()).toBe(true);
  });

  it('renders with empty list of proposals', async () => {
    const wrapper = mount(ProposalsPage);
    const pageBody = wrapper.find('[data-test-id="page-body"]');

    expect(pageBody.exists()).toBe(true);

    await wrapper.vm.$nextTick();

    expect(pageBody.find('[data-test-id="loading"]').exists()).toBe(true);

    await flushPromises();

    expect(pageBody.find('[data-test-id="proposals-empty-list"]').exists()).toBe(true);
  });

  it('renders with proposal list', async () => {
    vi.spyOn(mockedWalletService, 'listProposals').mockReturnValue(
      Promise.resolve({
        proposals: [
          {
            id: '1',
            created_at: new Date().toISOString(),
            operation: {
              AddUserGroup: {
                input: {
                  name: 'finance',
                },
              },
            },
            info: {
              can_vote: true,
            },
          },
        ],
        next_offset: [BigInt(1)],
        total: BigInt(2),
      } as ExtractOk<ListProposalsResult>),
    );

    const wrapper = mount(ProposalsPage);
    const pageBody = wrapper.find('[data-test-id="page-body"]');

    expect(pageBody.exists()).toBe(true);

    await wrapper.vm.$nextTick();

    expect(pageBody.find('[data-test-id="loading"]').exists()).toBe(true);

    await flushPromises();

    expect(pageBody.find('[data-test-id="loading"]').exists()).toBe(false);
    expect(pageBody.find('[data-test-id="proposals-empty-list"]').exists()).toBe(false);
  });
});
