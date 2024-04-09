import { describe, expect, it, vi } from 'vitest';
import { mockRouter, mount } from '~/test.utils';
import DeployWallet from './DeployWallet.vue';
import { services } from '~/plugins/services.plugin';
import { flushPromises } from '@vue/test-utils';
import { User } from '~/generated/control-panel/control_panel.did';
import { Principal } from '@dfinity/principal';

vi.mock('~/utils/helper.utils', async importOriginal => {
  const mod = (await importOriginal()) as object;
  return {
    ...mod,
    wait: vi.fn(),
  };
});

vi.mock('~/stores/wallet.store', async importOriginal => {
  const mod = (await importOriginal()) as object;
  return {
    ...mod,
    createUserInitialAccount: () => Promise.resolve(),
  };
});

describe('DeployWallet', () => {
  it('renders correctly', () => {
    vi.spyOn(services().controlPanel, 'getCurrentUser').mockResolvedValueOnce({} as User);
    const wrapper = mount(DeployWallet);
    expect(wrapper.exists()).toBe(true);
  });

  it('checks waitlist status on mount', async () => {
    vi.spyOn(services().controlPanel, 'getCurrentUser').mockResolvedValueOnce({} as User);
    const controlPanelSerivce = services().controlPanel;
    mount(DeployWallet);

    expect(controlPanelSerivce.getCurrentUser).toHaveBeenCalled();
  });

  it('shows error if check fails', async () => {
    vi.spyOn(services().controlPanel, 'getCurrentUser').mockRejectedValueOnce({});
    const wrapper = mount(DeployWallet);

    await flushPromises();

    expect(wrapper.find('[data-test-id="join-waitlist-check-error"]').exists()).toBe(true);
  });

  it('will show the waitlist screen if the user is not on the waitlist', async () => {
    vi.spyOn(services().controlPanel, 'getCurrentUser').mockResolvedValueOnce({
      subscription_status: { Unsubscribed: null },
    } as User);
    const wrapper = mount(DeployWallet);

    await flushPromises();

    expect(wrapper.find('[data-test-id="join-waitlist-form"]').exists()).toBe(true);
    expect(wrapper.find('[data-test-id="join-waitlist-form-email"]').exists()).toBe(true);
  });

  it("will show the waitlist pending screen if the user's waitlist status is still pending", async () => {
    vi.spyOn(services().controlPanel, 'getCurrentUser').mockResolvedValueOnce({
      subscription_status: { Pending: null },
    } as User);
    const wrapper = mount(DeployWallet);

    await flushPromises();

    expect(wrapper.find('[data-test-id="join-waitlist-pending"]').exists()).toBe(true);
  });

  it("will show the waitlist denied screen if the user's waitlist status is denied", async () => {
    vi.spyOn(services().controlPanel, 'getCurrentUser').mockResolvedValueOnce({
      subscription_status: { Denylisted: null },
    } as User);
    const wrapper = mount(DeployWallet);

    await flushPromises();

    expect(wrapper.find('[data-test-id="join-waitlist-denied"]').exists()).toBe(true);
  });

  it("will show the deploy screen if the user's waitlist status is approved", async () => {
    vi.spyOn(services().controlPanel, 'getCurrentUser').mockResolvedValue({
      subscription_status: { Approved: null },
    } as User);

    vi.spyOn(services().controlPanel, 'deployWallet').mockResolvedValueOnce(Principal.anonymous());
    vi.spyOn(services().wallet, 'isHealthy').mockResolvedValueOnce(true);
    const mockPush = vi.spyOn(mockRouter, 'push');

    const wrapper = mount(DeployWallet);

    await flushPromises();

    expect(wrapper.find('[data-test-id="deploying-wallet"]').exists()).toBe(true);

    // will redirect after deploy is complete
    expect(mockPush).toHaveBeenCalled();
  });
});
