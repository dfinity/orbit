import { describe, expect, it } from 'vitest';
import { mount } from '~/test.utils';
import SystemUpgradeActionBtn from './SystemUpgradeActionBtn.vue';

describe('SystemUpgradeActionBtn', () => {
  it('renders action btn', () => {
    const wrapper = mount(SystemUpgradeActionBtn);

    expect(wrapper.exists()).toBe(true);
  });

  it('on click emits editing', async () => {
    const wrapper = mount(SystemUpgradeActionBtn);
    const btn = wrapper.find('[data-test-id="submit-upgrade-btn"]');

    expect(btn.exists()).toBe(true);

    await btn.trigger('click');

    expect(wrapper.emitted('editing')).toBeTruthy();
  });

  it('highlight mode on mobile shows no text', async () => {
    const wrapper = mount(
      SystemUpgradeActionBtn,
      {
        props: {
          mode: 'highlight',
        },
      },
      {
        initialPiniaState: {
          app: {
            isMobile: true,
          },
        },
      },
    );

    const btn = wrapper.find('[data-test-id="submit-upgrade-btn"]');
    expect(btn.text()).toBe('');
  });

  it('highlight mode on desktop shows text', async () => {
    const wrapper = mount(
      SystemUpgradeActionBtn,
      {
        props: {
          mode: 'highlight',
        },
      },
      {
        initialPiniaState: {
          app: {
            isMobile: false,
          },
        },
      },
    );

    const btn = wrapper.find('[data-test-id="submit-upgrade-btn"]');
    expect(btn.text().length > 0).toBe(true);
  });
});
