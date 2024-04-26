import { describe, expect, it, vi } from 'vitest';
import { mount } from '~/test.utils';
import TransferDialog from './TransferDialog.vue';
import { Account, GetProposalResult, Proposal, Transfer } from '~/generated/station/station.did';
import { flushPromises } from '@vue/test-utils';
import { services } from '~/plugins/services.plugin';
import { ExtractOk } from '~/types/helper.types';

vi.mock('~/services/station.service', () => ({
  StationService: vi.fn().mockImplementation(() => {
    return {
      transfer: vi.fn(() => {
        return Promise.resolve({} as Proposal);
      }),
    };
  }),
}));

describe('TransferDialog', () => {
  it('renders correctly', () => {
    const wrapper = mount(TransferDialog, {
      props: {
        account: {
          id: '1',
          decimals: 1,
        } as Account,
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
          decimals: 1,
        } as Account,
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
    const summary = form.find(`[data-test-id="transfer-dialog-proposal-summary"]`);

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

  it('creates transfer proposal with summary', async () => {
    const wrapper = mount(TransferDialog, {
      props: {
        account: {
          id: '1',
          decimals: 1,
        } as Account,
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
    const summary = form.find(`[data-test-id="transfer-dialog-proposal-summary"]`);

    const submitButton = form.find(`[data-test-id="transfer-dialog-save-button"]`);

    amount.find('input').setValue('1');
    destination.find('input').setValue('destination address');
    summary.find('input').setValue('test summary');

    await flushPromises();

    await submitButton.trigger('click');

    await wrapper.vm.$nextTick();
    await flushPromises();

    expect(services().station.transfer).toHaveBeenCalledWith(
      expect.objectContaining({
        amount: 10n,
        to: 'destination address',
      }),
      'test summary',
    );
  });

  it('loads the corresponding objects to display the transfer and summary if transferId is specified', async () => {
    services().station.getProposal = vi.fn(() =>
      Promise.resolve({
        proposal: {
          summary: ['test summary'], // it's an opt
        } as unknown as Proposal,
      } as ExtractOk<GetProposalResult>),
    );
    services().station.getTransfer = vi.fn(() =>
      Promise.resolve({
        id: 'transfer-id',
        to: 'destination address',
        amount: 123n,
        proposal_id: 'proposal-id',
      } as Transfer),
    );

    const wrapper = mount(TransferDialog, {
      props: {
        account: {
          id: '1',
          decimals: 2,
        } as Account,
        open: true,
        transferId: 'transfer-id',
      },
    });
    await wrapper.vm.$nextTick();
    await flushPromises();
    await wrapper.vm.$nextTick();

    expect(services().station.getTransfer).toHaveBeenCalledWith('transfer-id');
    expect(services().station.getProposal).toHaveBeenCalledWith({
      proposal_id: 'proposal-id',
    });

    const dataLoader = wrapper.findComponent({ name: 'DataLoader' });
    const form = dataLoader.find(`[data-test-id="transfer-dialog-form"]`);

    const transferId = form.find(`[data-test-id="transfer-form-transfer-id"]`);
    const amount = form.find(`[data-test-id="transfer-form-amount"]`);
    const destination = form.find(`[data-test-id="transfer-form-destination-address"]`);
    const summary = form.find(`[data-test-id="transfer-dialog-proposal-summary"]`);

    expect(transferId.exists()).toBe(true);
    expect(transferId.find('input').element.value).toBe('transfer-id');

    expect(amount.find('input').element.value).toBe('1.23');
    expect(destination.find('input').element.value).toBe('destination address');
    expect(summary.find('input').element.value).toBe('test summary');
  });
});
