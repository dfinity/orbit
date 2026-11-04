import { describe, expect, it, vi } from 'vitest';
import { VBtn, VCard, VProgressCircular } from 'vuetify/components';
import { useSessionStore } from '~/stores/session.store';
import { mount } from '~/test.utils';
import SessionExpiredOverlay from './SessionExpiredOverlay.vue';

describe('SessionExpiredOverlay', () => {
  it('does not show up by default', () => {
    const wrapper = mount(SessionExpiredOverlay);
    expect(wrapper.findComponent(VCard).exists()).toBe(false);
  });

  it('shows up when reauthentication is needed', async () => {
    const wrapper = mount(SessionExpiredOverlay);
    const sessionStore = useSessionStore();

    expect(wrapper.findComponent(VCard).exists()).toBe(false);

    sessionStore.$patch({
      reauthenticationNeeded: true,
    });

    await wrapper.vm.$nextTick();

    expect(wrapper.findComponent(VCard).exists()).toBe(true);
  });

  it('calls signIn on reauthenticate button click', async () => {
    const wrapper = mount(SessionExpiredOverlay);
    const sessionStore = useSessionStore();

    sessionStore.signIn = vi.fn(() => Promise.resolve());

    sessionStore.$patch({
      reauthenticationNeeded: true,
    });
    await wrapper.vm.$nextTick();

    const reauthenticateButton = wrapper.getComponent(VBtn);

    reauthenticateButton.trigger('click');

    expect(sessionStore.signIn).toHaveBeenCalled();
  });

  it('shows a spinner while authenticating', async () => {
    const wrapper = mount(SessionExpiredOverlay);
    const sessionStore = useSessionStore();
    // The spinner only exists while signIn is in flight, so the promise is resolved by hand
    // after the assertion instead of resolving on the next microtask.
    let completeSignIn!: () => void;
    sessionStore.signIn = vi.fn(() => new Promise<void>(resolve => (completeSignIn = resolve)));
    sessionStore.$patch({
      reauthenticationNeeded: true,
    });
    await wrapper.vm.$nextTick();

    const reauthenticateButton = wrapper.getComponent(VBtn);

    await reauthenticateButton.trigger('click');

    const spinner = wrapper.findComponent(VProgressCircular);

    expect(spinner.exists()).toBe(true);

    completeSignIn();
  });
});
