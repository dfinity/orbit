import { describe, expect, it } from 'vitest';
import { mount } from '~/test.utils';
import AddWalletScreen from './AddWalletScreen.vue';

describe('AddWalletScreen', () => {
  it('renders correctly', () => {
    const wrapper = mount(AddWalletScreen);
    expect(wrapper.exists()).toBe(true);
  });

  it('shows the join wallet screen when selected', async () => {
    const wrapper = mount(AddWalletScreen);
    await wrapper.find('[data-test-id="join-existing-wallet-radio"] input').setValue(true);
    await wrapper.find('[data-test-id="continue-button"]').trigger('click');

    expect(wrapper.find('[data-test-id="join-wallet-screen"]').exists()).toBe(true);
  });
  it('can go back from the join wallet screen', async () => {
    const wrapper = mount(AddWalletScreen);
    await wrapper.find('[data-test-id="join-existing-wallet-radio"] input').setValue(true);
    await wrapper.find('[data-test-id="continue-button"]').trigger('click');

    expect(wrapper.find('[data-test-id="join-wallet-screen"]').exists()).toBe(true);

    await wrapper.find('[data-test-id="back-button"]').trigger('click');
    expect(wrapper.find('[data-test-id="split-screen"]').exists()).toBe(true);
  });
  it('shows the deploy wallet screen when selected', async () => {
    const wrapper = mount(AddWalletScreen);

    await wrapper.find('[data-test-id="deploy-new-wallet-radio"] input').setValue(true);
    await wrapper.find('[data-test-id="continue-button"]').trigger('click');

    expect(wrapper.find('[data-test-id="deploy-wallet-screen"]').exists()).toBe(true);
  });
  it('can go back from the deploy wallet screen', async () => {
    const wrapper = mount(AddWalletScreen);
    await wrapper.find('[data-test-id="deploy-new-wallet-radio"] input').setValue(true);
    await wrapper.find('[data-test-id="continue-button"]').trigger('click');
    expect(wrapper.find('[data-test-id="deploy-wallet-screen"]').exists()).toBe(true);

    await wrapper.find('[data-test-id="back-button"]').trigger('click');
    expect(wrapper.find('[data-test-id="split-screen"]').exists()).toBe(true);
  });
});
