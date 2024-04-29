import { describe, expect, it } from 'vitest';
import { mockRouter, mount } from '~/test.utils';
import AddStationListItem from './AddStationListItem.vue';
import { flushPromises } from '@vue/test-utils';

describe('AddStationForm', () => {
  it('renders correctly', () => {
    const wrapper = mount(AddStationListItem);
    expect(wrapper.exists()).toBe(true);
  });

  it('goes to /add-station when clicked', async () => {
    const listItem = mount(AddStationListItem);

    const button = listItem.find('[data-test-id="add-station-item"]');
    expect(button.exists()).toBe(true);

    await button.trigger('click');
    await flushPromises();

    expect(mockRouter.currentRoute.value.params.pathMatch).toContain('add-station');
  });
});
