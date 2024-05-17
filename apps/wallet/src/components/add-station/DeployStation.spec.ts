import { describe, expect, it, vi } from 'vitest';
import { mockRouter, mount } from '~/test.utils';
import DeployStation from './DeployStation.vue';
import { services } from '~/plugins/services.plugin';
import { flushPromises } from '@vue/test-utils';
import { CanDeployStationResponse, User } from '~/generated/control-panel/control_panel.did';
import { Principal } from '@dfinity/principal';
import { StationService } from '~/services/station.service';
import { ControlPanelService } from '~/services/control-panel.service';

vi.mock('~/utils/helper.utils', async importOriginal => {
  const mod = (await importOriginal()) as object;
  return {
    ...mod,
    wait: vi.fn(),
  };
});

vi.mock('~/stores/station.store', async importOriginal => {
  const mod = (await importOriginal()) as object;
  return {
    ...mod,
    createUserInitialAccount: () => Promise.resolve(),
  };
});

vi.mock('~/services/control-panel.service', () => {
  const mock: Partial<ControlPanelService> = {
    listUserStations: vi.fn().mockResolvedValue([]),
    canDeployStation: vi.fn().mockReturnValue({}),
    getCurrentUser: vi.fn().mockResolvedValue({}),
    deployStation: vi.fn().mockResolvedValue({}),
  };

  return {
    ControlPanelService: vi.fn(() => mock),
  };
});

vi.mock('~/services/station.service', () => {
  const mock: Partial<StationService> = {
    withStationId: vi.fn().mockReturnThis(),
    capabilities: vi.fn().mockImplementation(() => Promise.resolve({})),
    isHealthy: vi.fn().mockResolvedValue(true),
  };

  return {
    StationService: vi.fn(() => mock),
  };
});

describe('DeployStation', () => {
  it('renders correctly', () => {
    vi.spyOn(services().controlPanel, 'canDeployStation').mockResolvedValueOnce(
      {} as CanDeployStationResponse,
    );
    const wrapper = mount(DeployStation);
    expect(wrapper.exists()).toBe(true);
  });

  it('checks can deploy status on mount', async () => {
    vi.spyOn(services().controlPanel, 'canDeployStation').mockResolvedValueOnce(
      {} as CanDeployStationResponse,
    );
    const controlPanelSerivce = services().controlPanel;
    mount(DeployStation);

    expect(controlPanelSerivce.canDeployStation).toHaveBeenCalled();
  });

  it('shows error if check fails', async () => {
    vi.spyOn(services().controlPanel, 'canDeployStation').mockRejectedValueOnce({});
    const wrapper = mount(DeployStation);

    await flushPromises();

    expect(wrapper.find('[data-test-id="deploy-check-error"]').exists()).toBe(true);
  });

  it('will show the waitlist screen if the user is not on the waitlist', async () => {
    vi.spyOn(services().controlPanel, 'canDeployStation').mockResolvedValueOnce({
      NotAllowed: { Unsubscribed: null },
    } as CanDeployStationResponse);
    const wrapper = mount(DeployStation);

    await flushPromises();

    expect(wrapper.find('[data-test-id="join-waitlist-form"]').exists()).toBe(true);
    expect(wrapper.find('[data-test-id="join-waitlist-form-email"]').exists()).toBe(true);
  });

  it("will show the pending screen if the user's waitlist status is still pending", async () => {
    vi.spyOn(services().controlPanel, 'canDeployStation').mockResolvedValueOnce({
      NotAllowed: { Pending: null },
    } as CanDeployStationResponse);
    const wrapper = mount(DeployStation);

    await flushPromises();

    expect(wrapper.find('[data-test-id="deploy-in-waiting-list"]').exists()).toBe(true);
  });

  it("will show the denied screen if the user's waitlist status is denied", async () => {
    vi.spyOn(services().controlPanel, 'canDeployStation').mockResolvedValueOnce({
      NotAllowed: { Denylisted: null },
    } as CanDeployStationResponse);
    const wrapper = mount(DeployStation);

    await flushPromises();

    expect(wrapper.find('[data-test-id="deploy-not-allowed"]').exists()).toBe(true);
  });

  it('will show the quota exceedeed screen if the has already deployed too many wallets', async () => {
    vi.spyOn(services().controlPanel, 'canDeployStation').mockResolvedValueOnce({
      QuotaExceeded: null,
    } as CanDeployStationResponse);
    const wrapper = mount(DeployStation);

    await flushPromises();

    expect(wrapper.find('[data-test-id="deploy-quota-exceeded-error"]').exists()).toBe(true);
  });

  it('will show the deploy screen if the user is approved to create a new wallet', async () => {
    vi.spyOn(services().controlPanel, 'canDeployStation').mockResolvedValue({
      Allowed: BigInt(10),
    } as CanDeployStationResponse);
    vi.spyOn(services().controlPanel, 'getCurrentUser').mockResolvedValue({
      subscription_status: { Approved: null },
    } as User);

    vi.spyOn(services().controlPanel, 'deployStation').mockResolvedValueOnce(Principal.anonymous());
    vi.spyOn(services().station, 'isHealthy').mockResolvedValueOnce(true);
    const mockPush = vi.spyOn(mockRouter, 'push');

    const wrapper = mount(DeployStation);

    await flushPromises();

    const form = wrapper.find('[data-test-id="deploy-station-form"]');

    form.find('input[name="station_name"]').setValue('test');
    form.find('input[name="admin_name"]').setValue('admin');

    await flushPromises();

    form.trigger('submit');

    await wrapper.vm.$nextTick();
    await flushPromises();

    expect(wrapper.find('[data-test-id="deploying-station"]').exists()).toBe(true);

    // will redirect after deploy is complete
    expect(mockPush).toHaveBeenCalled();
  });
});
