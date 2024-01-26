import { describe, expect, it } from 'vitest';
import { mount } from '../../test.utils';
import AddWalletListItem from './AddWalletListItem.vue';
import AddWalletDialogVue from './AddWalletDialog.vue';

describe('AddWalletForm', () => {
  it('renders correctly', () => {
    const wrapper = mount(AddWalletListItem);
    expect(wrapper.exists()).toBe(true);
  });

  it('opens the Add wallet dialog when clicked', async () => {
    const listItem = mount(AddWalletListItem);

    const button = listItem.find('[data-test-id="add-wallet-item"]');
    expect(button.exists()).toBe(true);

    await button.trigger('click');
    await listItem.vm.$nextTick();

    const dialog = listItem.findComponent(AddWalletDialogVue);
    expect(dialog.exists()).toBe(true);
  });
});
