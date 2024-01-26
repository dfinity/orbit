import { describe, expect, it } from 'vitest';
import { VList, VSelect } from 'vuetify/components';
import { mount } from '../test.utils';
import WalletSelector from './WalletSelector.vue';

describe('WalletSelector', () => {
  it('renders correctly', () => {
    const wrapper = mount(WalletSelector);
    expect(wrapper.exists()).toBe(true);
  });

  describe('Add wallet button', () => {
    it('exists in the dropdown', async () => {
      const wrapper = mount(WalletSelector);
      const select = wrapper.findComponent(VSelect);
      await select.trigger('click');

      await wrapper.vm.$nextTick();

      const menu = wrapper.findComponent(VList);

      expect(menu.find('[data-test-id="add-wallet-item"]').exists()).toBe(true);
    });
  });
});
