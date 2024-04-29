import { describe, expect, it, vi } from 'vitest';
import { mount } from '~/test.utils';
import JoinStation from './JoinStation.vue';
import { useSessionStore } from '~/stores/session.store';
import { flushPromises } from '@vue/test-utils';
import { useAppStore } from '~/stores/app.store';

describe('JoinStation', () => {
  it('renders correctly', () => {
    const wrapper = mount(JoinStation);
    expect(wrapper.exists()).toBe(true);
  });

  it('shows a form with a Name and Canister Id field', async () => {
    const wrapper = mount(JoinStation);

    expect(wrapper.find('[data-test-id="join-station-form-canister-name"]').exists()).toBe(true);
    expect(wrapper.find('[data-test-id="join-station-form-canister-id"]').exists()).toBe(true);
  });

  it('has a submit button', async () => {
    const wrapper = mount(JoinStation);

    const submit = wrapper.find('button[type="submit"]');
    expect(submit.exists()).toBe(true);
  });

  it('wont submit empty form which is invalid', async () => {
    const wrapper = mount(JoinStation);
    const sessionStore = useSessionStore();

    const submit = wrapper.find('button[type="submit"]');

    submit.trigger('click');
    await flushPromises();

    expect(sessionStore.addStation).not.toHaveBeenCalled();
  });

  it('wont submit the form if its invalid', async () => {
    const wrapper = mount(JoinStation);

    await wrapper.findComponent({ ref: 'form' }).trigger('submit');
    await flushPromises();

    const canisterId = wrapper.find('[data-test-id="join-station-form-canister-id"] input');

    // name is optional
    expect(
      wrapper.find('[data-test-id="join-station-form-canister-name"]').classes(),
    ).not.toContain('v-input--error');

    // canister id is required
    expect(wrapper.find('[data-test-id="join-station-form-canister-id"]').classes()).toContain(
      'v-input--error',
    );

    await canisterId.setValue('hello');
    await flushPromises();

    expect(wrapper.find('[data-test-id="join-station-form-canister-id"]').classes()).toContain(
      'v-input--error',
    );

    await canisterId.setValue('bd3sg-teaaa-aaaaa-qaaba-cai');
    await flushPromises();

    expect(wrapper.find('[data-test-id="join-station-form-canister-id"]').classes()).not.toContain(
      'v-input--error',
    );
  });

  it('will call sessionStore.addStation on submit if the form is valid', async () => {
    const wrapper = mount(JoinStation);
    const sessionStore = useSessionStore();

    const name = wrapper.find('[data-test-id="join-station-form-canister-name"] input');
    const canisterId = wrapper.find('[data-test-id="join-station-form-canister-id"] input');

    await name.setValue('test');
    await canisterId.setValue('bd3sg-teaaa-aaaaa-qaaba-cai');

    await wrapper.findComponent({ ref: 'form' }).trigger('submit');

    await flushPromises();

    expect(sessionStore.addStation).toHaveBeenCalled();

    vi.resetAllMocks();
  });

  it('will show a spinner during submission', async () => {
    const wrapper = mount(JoinStation);
    const sessionStore = useSessionStore();

    // make sure the addStation call never resolves so we can test the spinner
    sessionStore.addStation = vi.fn(async () => new Promise<void>(() => {}));

    const name = wrapper.find('[data-test-id="join-station-form-canister-name"] input');
    const canisterId = wrapper.find('[data-test-id="join-station-form-canister-id"] input');

    await name.setValue('test');
    await canisterId.setValue('bd3sg-teaaa-aaaaa-qaaba-cai');

    await wrapper.findComponent({ ref: 'form' }).trigger('submit');

    await flushPromises();

    await vi.waitFor(() => expect(sessionStore.addStation).toHaveBeenCalled());

    expect(wrapper.find('button[type="submit"]').classes()).toContain('v-btn--loading');
    vi.resetAllMocks();
  });

  it('will refresh the station list on successful submission', async () => {
    const wrapper = mount(JoinStation);
    const sessionStore = useSessionStore();

    const name = wrapper.find('[data-test-id="join-station-form-canister-name"] input');
    const canisterId = wrapper.find('[data-test-id="join-station-form-canister-id"] input');

    await name.setValue('test');
    await canisterId.setValue('bd3sg-teaaa-aaaaa-qaaba-cai');

    await wrapper.findComponent({ ref: 'form' }).trigger('submit');

    await flushPromises();

    expect(sessionStore.addStation).toHaveBeenCalled();
    vi.resetAllMocks();
  });

  it('will show an error message on submission error', async () => {
    const wrapper = mount(JoinStation);
    const sessionStore = useSessionStore();

    sessionStore.addStation = vi.fn(() => Promise.reject());

    const app = useAppStore();
    const name = wrapper.find('[data-test-id="join-station-form-canister-name"] input');
    const canisterId = wrapper.find('[data-test-id="join-station-form-canister-id"] input');

    await name.setValue('test');
    await canisterId.setValue('bd3sg-teaaa-aaaaa-qaaba-cai');
    await wrapper.findComponent({ ref: 'form' }).trigger('submit');

    await flushPromises();

    expect(sessionStore.addStation).toHaveBeenCalled();
    expect(app.sendErrorNotification).toHaveBeenCalled();

    vi.resetAllMocks();
  });
});
