import { describe, expect, it } from 'vitest';
import { VBtn, VCard } from 'vuetify/components';
import { mount } from '~/ui/test.utils';
import { useSessionStore } from '../stores/session';
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
    sessionStore.$patch({
      reauthenticationNeeded: true,
    });
    await wrapper.vm.$nextTick();

    const reauthenticateButton = wrapper.getComponent(VBtn);

    reauthenticateButton.trigger('click');

    expect(sessionStore.signIn).toHaveBeenCalled();
  });

  it('calls signIn on reauthenticate button click', async () => {
    const wrapper = mount(SessionExpiredOverlay);
    const sessionStore = useSessionStore();
    sessionStore.$patch({
      reauthenticationNeeded: true,
    });
    await wrapper.vm.$nextTick();

    const reauthenticateButton = wrapper.getComponent(VBtn);

    reauthenticateButton.trigger('click');

    expect(sessionStore.signIn).toHaveBeenCalled();
  });
});
