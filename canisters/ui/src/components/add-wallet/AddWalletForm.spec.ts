import { describe, expect, it, vi } from 'vitest';
import { mount } from '~/test.utils';
import AddWalletForm from './AddWalletForm.vue';
import { flushPromises } from '@vue/test-utils';
import { User } from '~/generated/control-panel/control_panel.did';
import { services } from '~/modules/services.module';
import { useAppStore } from '~/stores/app.store';
import { useSessionStore } from '~/stores/session.store';

vi.mock('~/services/control-panel.service', () => ({
  ControlPanelService: vi.fn().mockImplementation(() => {
    return {
      editUser: vi.fn(() => {
        return Promise.resolve({} as User);
      }),
    };
  }),
}));

describe('AddWalletForm', () => {
  it('renders correctly', () => {
    const wrapper = mount(AddWalletForm);
    expect(wrapper.exists()).toBe(true);
  });

  it('shows a form with a Name and Canister Id field', async () => {
    const wrapper = mount(AddWalletForm);

    expect(wrapper.find('[data-test-id="add-wallet-form-name"]').exists()).toBe(true);
    expect(wrapper.find('[data-test-id="add-wallet-form-canister-id"]').exists()).toBe(true);
  });

  it('has a submit button', async () => {
    const wrapper = mount(AddWalletForm);

    const submit = wrapper.find('button[type="submit"]');
    expect(submit.exists()).toBe(true);
  });

  it('wont submit the form if its invalid', async () => {
    const wrapper = mount(AddWalletForm);

    const submit = wrapper.find('button[type="submit"]');

    submit.trigger('click');

    expect(wrapper.emitted('submit')).toBeUndefined();
  });

  it('wont submit the form if its invalid', async () => {
    const wrapper = mount(AddWalletForm);

    await wrapper.findComponent({ ref: 'form' }).trigger('submit');
    await flushPromises();
    expect(wrapper.emitted('submitted')).toBeUndefined();

    const canisterId = wrapper.find('[data-test-id="add-wallet-form-canister-id"] input');

    // name is optional
    expect(wrapper.find('[data-test-id="add-wallet-form-name"]').classes()).not.toContain(
      'v-input--error',
    );

    // canister id is required
    expect(wrapper.find('[data-test-id="add-wallet-form-canister-id"]').classes()).toContain(
      'v-input--error',
    );

    await canisterId.setValue('hello');
    await flushPromises();

    expect(wrapper.find('[data-test-id="add-wallet-form-canister-id"]').classes()).toContain(
      'v-input--error',
    );

    await canisterId.setValue('bd3sg-teaaa-aaaaa-qaaba-cai');
    await flushPromises();

    expect(wrapper.find('[data-test-id="add-wallet-form-canister-id"]').classes()).not.toContain(
      'v-input--error',
    );
  });

  it('will call ControlPanelService.editUser on submit if the form is valid', async () => {
    const wrapper = mount(AddWalletForm);

    const name = wrapper.find('[data-test-id="add-wallet-form-name"] input');
    const canisterId = wrapper.find('[data-test-id="add-wallet-form-canister-id"] input');

    await name.setValue('test');
    await canisterId.setValue('bd3sg-teaaa-aaaaa-qaaba-cai');

    await wrapper.findComponent({ ref: 'form' }).trigger('submit');

    await flushPromises();

    expect(services().controlPanel.editUser).toHaveBeenCalled();

    expect(wrapper.emitted('submitted')).toBeTruthy();

    vi.resetAllMocks();
  });

  it('will show a spinner during submission', async () => {
    services().controlPanel.editUser = vi.fn(
      // make sure the editUser call never resolves so we can test the spinner
      async () => new Promise<User>(() => {}),
    );

    const wrapper = mount(AddWalletForm);

    const name = wrapper.find('[data-test-id="add-wallet-form-name"] input');
    const canisterId = wrapper.find('[data-test-id="add-wallet-form-canister-id"] input');

    await name.setValue('test');
    await canisterId.setValue('bd3sg-teaaa-aaaaa-qaaba-cai');

    await wrapper.findComponent({ ref: 'form' }).trigger('submit');

    await vi.waitFor(() => expect(services().controlPanel.editUser).toHaveBeenCalled());

    expect(wrapper.find('button[type="submit"]').classes()).toContain('v-btn--loading');
    vi.resetAllMocks();
  });

  it('will refresh the wallet list on successful submission', async () => {
    const wrapper = mount(AddWalletForm);
    const sessionStore = useSessionStore();

    const name = wrapper.find('[data-test-id="add-wallet-form-name"] input');
    const canisterId = wrapper.find('[data-test-id="add-wallet-form-canister-id"] input');

    await name.setValue('test');
    await canisterId.setValue('bd3sg-teaaa-aaaaa-qaaba-cai');

    await wrapper.findComponent({ ref: 'form' }).trigger('submit');

    await flushPromises();

    expect(services().controlPanel.editUser).toHaveBeenCalled();
    expect(sessionStore.populateUser).toHaveBeenCalled();
  });

  it('will show an error message on submission error', async () => {
    services().controlPanel.editUser = vi.fn(() =>
      Promise.reject({
        code: 'code',
        message: ['test message'],
        details: [],
      }),
    );

    const wrapper = mount(AddWalletForm);
    const app = useAppStore();
    const name = wrapper.find('[data-test-id="add-wallet-form-name"] input');
    const canisterId = wrapper.find('[data-test-id="add-wallet-form-canister-id"] input');

    await name.setValue('test');
    await canisterId.setValue('bd3sg-teaaa-aaaaa-qaaba-cai');
    await wrapper.findComponent({ ref: 'form' }).trigger('submit');

    await flushPromises();

    expect(services().controlPanel.editUser).toHaveBeenCalled();

    // partial match because `message` can be arbitrary
    expect(app.sendNotification).toHaveBeenCalledWith(
      expect.objectContaining({
        type: 'error',
      }),
    );

    expect(wrapper.emitted('submitted')).toBeFalsy();

    vi.resetAllMocks();
  });
});
