import { describe, expect, it, vi } from 'vitest';
import { mount } from '~/test.utils';
import JoinWallet from './JoinWallet.vue';
import { useSessionStore } from '~/stores/session.store';
import { flushPromises } from '@vue/test-utils';
import { useAppStore } from '~/stores/app.store';

describe('JoinWallet', () => {
  it('renders correctly', () => {
    const wrapper = mount(JoinWallet);
    expect(wrapper.exists()).toBe(true);
  });

  it('shows a form with a Name and Canister Id field', async () => {
    const wrapper = mount(JoinWallet);

    expect(wrapper.find('[data-test-id="join-wallet-form-canister-name"]').exists()).toBe(true);
    expect(wrapper.find('[data-test-id="join-wallet-form-canister-id"]').exists()).toBe(true);
  });

  it('has a submit button', async () => {
    const wrapper = mount(JoinWallet);

    const submit = wrapper.find('button[type="submit"]');
    expect(submit.exists()).toBe(true);
  });

  it('wont submit empty form which is invalid', async () => {
    const wrapper = mount(JoinWallet);
    const sessionStore = useSessionStore();

    const submit = wrapper.find('button[type="submit"]');

    submit.trigger('click');
    await flushPromises();

    expect(sessionStore.addWallet).not.toHaveBeenCalled();
  });

  it('wont submit the form if its invalid', async () => {
    const wrapper = mount(JoinWallet);

    await wrapper.findComponent({ ref: 'form' }).trigger('submit');
    await flushPromises();

    const canisterId = wrapper.find('[data-test-id="join-wallet-form-canister-id"] input');

    // name is optional
    expect(wrapper.find('[data-test-id="join-wallet-form-canister-name"]').classes()).not.toContain(
      'v-input--error',
    );

    // canister id is required
    expect(wrapper.find('[data-test-id="join-wallet-form-canister-id"]').classes()).toContain(
      'v-input--error',
    );

    await canisterId.setValue('hello');
    await flushPromises();

    expect(wrapper.find('[data-test-id="join-wallet-form-canister-id"]').classes()).toContain(
      'v-input--error',
    );

    await canisterId.setValue('bd3sg-teaaa-aaaaa-qaaba-cai');
    await flushPromises();

    expect(wrapper.find('[data-test-id="join-wallet-form-canister-id"]').classes()).not.toContain(
      'v-input--error',
    );
  });

  it('will call sessionStore.addWallet on submit if the form is valid', async () => {
    const wrapper = mount(JoinWallet);
    const sessionStore = useSessionStore();

    const name = wrapper.find('[data-test-id="join-wallet-form-canister-name"] input');
    const canisterId = wrapper.find('[data-test-id="join-wallet-form-canister-id"] input');

    await name.setValue('test');
    await canisterId.setValue('bd3sg-teaaa-aaaaa-qaaba-cai');

    await wrapper.findComponent({ ref: 'form' }).trigger('submit');

    await flushPromises();

    expect(sessionStore.addWallet).toHaveBeenCalled();

    vi.resetAllMocks();
  });

  it('will show a spinner during submission', async () => {
    const wrapper = mount(JoinWallet);
    const sessionStore = useSessionStore();

    // make sure the addWallet call never resolves so we can test the spinner
    sessionStore.addWallet = vi.fn(async () => new Promise<void>(() => {}));

    const name = wrapper.find('[data-test-id="join-wallet-form-canister-name"] input');
    const canisterId = wrapper.find('[data-test-id="join-wallet-form-canister-id"] input');

    await name.setValue('test');
    await canisterId.setValue('bd3sg-teaaa-aaaaa-qaaba-cai');

    await wrapper.findComponent({ ref: 'form' }).trigger('submit');

    await flushPromises();

    await vi.waitFor(() => expect(sessionStore.addWallet).toHaveBeenCalled());

    expect(wrapper.find('button[type="submit"]').classes()).toContain('v-btn--loading');
    vi.resetAllMocks();
  });

  it('will refresh the wallet list on successful submission', async () => {
    const wrapper = mount(JoinWallet);
    const sessionStore = useSessionStore();

    const name = wrapper.find('[data-test-id="join-wallet-form-canister-name"] input');
    const canisterId = wrapper.find('[data-test-id="join-wallet-form-canister-id"] input');

    await name.setValue('test');
    await canisterId.setValue('bd3sg-teaaa-aaaaa-qaaba-cai');

    await wrapper.findComponent({ ref: 'form' }).trigger('submit');

    await flushPromises();

    expect(sessionStore.addWallet).toHaveBeenCalled();
    vi.resetAllMocks();
  });

  it('will show an error message on submission error', async () => {
    const wrapper = mount(JoinWallet);
    const sessionStore = useSessionStore();

    sessionStore.addWallet = vi.fn(() => Promise.reject());

    const app = useAppStore();
    const name = wrapper.find('[data-test-id="join-wallet-form-canister-name"] input');
    const canisterId = wrapper.find('[data-test-id="join-wallet-form-canister-id"] input');

    await name.setValue('test');
    await canisterId.setValue('bd3sg-teaaa-aaaaa-qaaba-cai');
    await wrapper.findComponent({ ref: 'form' }).trigger('submit');

    await flushPromises();

    expect(sessionStore.addWallet).toHaveBeenCalled();
    expect(app.sendErrorNotification).toHaveBeenCalled();

    vi.resetAllMocks();
  });
});
