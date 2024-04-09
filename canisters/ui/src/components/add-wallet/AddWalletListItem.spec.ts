import { describe, expect, it } from 'vitest';
import { mockRouter, mount } from '~/test.utils';
import AddWalletListItem from './AddWalletListItem.vue';
import { flushPromises } from '@vue/test-utils';

describe('AddWalletForm', () => {
  it('renders correctly', () => {
    const wrapper = mount(AddWalletListItem);
    expect(wrapper.exists()).toBe(true);
  });

  it('goes to /add-wallet when clicked', async () => {
    const listItem = mount(AddWalletListItem);

    const button = listItem.find('[data-test-id="add-wallet-item"]');
    expect(button.exists()).toBe(true);

    await button.trigger('click');
    await flushPromises();

    expect(mockRouter.currentRoute.value.params.pathMatch).toContain('add-wallet');
  });
});
