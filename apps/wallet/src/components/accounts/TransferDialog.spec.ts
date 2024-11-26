import { describe, expect, it, vi } from 'vitest';
import { mount } from '~/test.utils';
import TransferDialog from './TransferDialog.vue';
import {
  Account,
  Asset,
  GetRequestResult,
  Request,
  Transfer,
} from '~/generated/station/station.did';
import { flushPromises } from '@vue/test-utils';
import { services } from '~/plugins/services.plugin';
import { ExtractOk } from '~/types/helper.types';
import { AddressFormat } from '~/types/chain.types';

const validIcpAddress = '5c76bc95e544204de4928e4d901e52b49df248b9c346807040e7af75aa61f4b3';

vi.mock('~/utils/asset.utils', () => ({
  detectAddressStandard: vi.fn(() => 'icp_native'),
  detectAddressFormat: vi.fn(() => AddressFormat.ICPNative),
}));

vi.mock('~/services/station.service', () => ({
  StationService: vi.fn().mockImplementation(() => {
    return {
      transfer: vi.fn(() => {
        return Promise.resolve({} as Request);
      }),
    };
  }),
}));

const mockAsset: Asset = {
  blockchain: 'icp',
  decimals: 2,
  id: '1',
  metadata: [],
  name: 'ICP',
  symbol: 'ICP',
  standards: ['icp_native', 'icrc1'],
};

describe('TransferDialog', () => {
  it('renders correctly', () => {
    const wrapper = mount(TransferDialog, {
      props: {
        account: {
          id: '1',
        } as Account,
        asset: mockAsset,
        open: true,
      },
    });
    expect(wrapper.exists()).toBe(true);
  });

  it('shows empty form when transferId not specified', async () => {
    const wrapper = mount(TransferDialog, {
      props: {
        account: {
          id: '1',
        } as Account,
        asset: mockAsset,
        open: true,
      },
    });
    await wrapper.vm.$nextTick();
    await flushPromises();
    await wrapper.vm.$nextTick();

    const dataLoader = wrapper.findComponent({ name: 'DataLoader' });
    const form = dataLoader.find(`[data-test-id="transfer-dialog-form"]`);

    const transferId = form.find(`[data-test-id="transfer-form-transfer-id"]`);
    const amount = form.find(`[data-test-id="transfer-form-amount"]`);
    const destination = form.find(`[data-test-id="transfer-form-destination-address"]`);
    const summary = form.find(`[data-test-id="transfer-dialog-request-summary"]`);

    // transferId should not be visible when not specified as a prop
    expect(transferId.exists()).toBe(false);

    expect(amount.exists()).toBe(true);
    expect(destination.exists()).toBe(true);
    expect(summary.exists()).toBe(true);

    // amount field should be empty
    expect(amount.find('input').element.value).toBe('');

    // destination field should be empty
    expect(destination.find('input').element.value).toBe('');

    // summary should be empty
    expect(summary.find('input').element.value).toBe('');
  });

  it('creates transfer request with summary', async () => {
    const wrapper = mount(TransferDialog, {
      props: {
        account: {
          id: '1',
        } as Account,
        asset: mockAsset,
        open: true,
      },
    });
    await wrapper.vm.$nextTick();
    await flushPromises();
    await wrapper.vm.$nextTick();

    const dataLoader = wrapper.findComponent({ name: 'DataLoader' });
    const form = dataLoader.find(`[data-test-id="transfer-dialog-form"]`);

    const amount = form.find(`[data-test-id="transfer-form-amount"]`);
    const destination = form.find(`[data-test-id="transfer-form-destination-address"]`);
    const summary = form.find(`[data-test-id="transfer-dialog-request-summary"]`);

    const submitButton = form.find(`[data-test-id="transfer-dialog-save-button"]`);

    await amount.find('input').setValue('1');
    await destination.find('input').setValue(validIcpAddress);
    await summary.find('input').setValue('test summary');

    await flushPromises();

    await submitButton.trigger('click');

    await flushPromises();

    expect(services().station.transfer).toHaveBeenCalledWith(
      expect.objectContaining({
        amount: 100n, // decimals are 2
        to: validIcpAddress,
      }),
      'test summary',
    );
  });

  it('loads the corresponding objects to display the transfer and summary if transferId is specified', async () => {
    services().station.getRequest = vi.fn(() =>
      Promise.resolve({
        request: {
          summary: ['test summary'], // it's an opt
        } as unknown as Request,
      } as ExtractOk<GetRequestResult>),
    );
    services().station.getTransfer = vi.fn(() =>
      Promise.resolve({
        id: 'transfer-id',
        to: validIcpAddress,
        amount: 123n,
        request_id: 'request-id',
      } as Transfer),
    );

    const wrapper = mount(TransferDialog, {
      props: {
        account: {
          id: '1',
        } as Account,
        asset: mockAsset,
        open: true,
        transferId: 'transfer-id',
      },
    });
    await wrapper.vm.$nextTick();
    await flushPromises();
    await wrapper.vm.$nextTick();

    expect(services().station.getTransfer).toHaveBeenCalledWith('transfer-id');
    expect(services().station.getRequest).toHaveBeenCalledWith({
      request_id: 'request-id',
      with_full_info: [],
    });

    const dataLoader = wrapper.findComponent({ name: 'DataLoader' });
    const form = dataLoader.find(`[data-test-id="transfer-dialog-form"]`);

    const transferId = form.find(`[data-test-id="transfer-form-transfer-id"]`);
    const amount = form.find(`[data-test-id="transfer-form-amount"]`);
    const destination = form.find(`[data-test-id="transfer-form-destination-address"]`);
    const summary = form.find(`[data-test-id="transfer-dialog-request-summary"]`);

    expect(transferId.exists()).toBe(true);
    expect(transferId.find('input').element.value).toBe('transfer-id');

    expect(amount.find('input').element.value).toBe('1.23');
    expect(destination.find('input').element.value).toBe(validIcpAddress);
    expect(summary.find('input').element.value).toBe('test summary');
  });
});
