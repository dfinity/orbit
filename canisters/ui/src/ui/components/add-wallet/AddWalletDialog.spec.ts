import { describe, expect, it } from 'vitest';
import { mount } from '../../test.utils';
import AddWalletDialog from './AddWalletDialog.vue';

describe('AddWalletDialog', () => {
  it('renders correctly', () => {
    const wrapper = mount(AddWalletDialog);
    expect(wrapper.exists()).toBe(true);
  });

  it.todo('shows the AddWalletForm', async () => {
    // todo: find a way to test this
  });
});
