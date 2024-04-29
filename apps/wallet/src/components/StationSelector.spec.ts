import { describe, expect, it } from 'vitest';
import { VList, VSelect } from 'vuetify/components';
import { mount } from '~/test.utils';
import StationSelector from './StationSelector.vue';

describe('StationSelector', () => {
  it('renders correctly', () => {
    const wrapper = mount(StationSelector);
    expect(wrapper.exists()).toBe(true);
  });

  describe('Add station button', () => {
    it('exists in the dropdown', async () => {
      const wrapper = mount(StationSelector);
      const select = wrapper.findComponent(VSelect);
      await select.trigger('click');

      await wrapper.vm.$nextTick();

      const menu = wrapper.findComponent(VList);

      expect(menu.find('[data-test-id="add-station-item"]').exists()).toBe(true);
    });
  });
});
