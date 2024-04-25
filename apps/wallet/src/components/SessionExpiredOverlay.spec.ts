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
    sessionStore.signIn = vi.fn(() => Promise.resolve());
    sessionStore.$patch({
      reauthenticationNeeded: true,
    });
    await wrapper.vm.$nextTick();

    const reauthenticateButton = wrapper.getComponent(VBtn);

    reauthenticateButton.trigger('click');

    await wrapper.vm.$nextTick();

    const spinner = wrapper.findComponent(VProgressCircular);

    expect(spinner.exists()).toBe(true);
  });
});
