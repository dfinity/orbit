import { describe, expect, it } from 'vitest';
import { mount } from '~/test.utils';
import AddStationScreen from './AddStationScreen.vue';

describe('AddStationScreen', () => {
  it('renders correctly', () => {
    const wrapper = mount(AddStationScreen);
    expect(wrapper.exists()).toBe(true);
  });

  it('shows the join station screen when selected', async () => {
    const wrapper = mount(AddStationScreen);
    await wrapper.find('[data-test-id="join-existing-station-radio"] input').setValue(true);
    await wrapper.find('[data-test-id="continue-button"]').trigger('click');

    expect(wrapper.find('[data-test-id="join-station-screen"]').exists()).toBe(true);
  });
  it('can go back from the join station screen', async () => {
    const wrapper = mount(AddStationScreen);
    await wrapper.find('[data-test-id="join-existing-station-radio"] input').setValue(true);
    await wrapper.find('[data-test-id="continue-button"]').trigger('click');

    expect(wrapper.find('[data-test-id="join-station-screen"]').exists()).toBe(true);

    await wrapper.find('[data-test-id="back-button"]').trigger('click');
    expect(wrapper.find('[data-test-id="split-screen"]').exists()).toBe(true);
  });
  it('shows the deploy station screen when selected', async () => {
    const wrapper = mount(AddStationScreen);

    await wrapper.find('[data-test-id="deploy-new-station-radio"] input').setValue(true);
    await wrapper.find('[data-test-id="continue-button"]').trigger('click');

    expect(wrapper.find('[data-test-id="deploy-station-screen"]').exists()).toBe(true);
  });
  it('can go back from the deploy station screen', async () => {
    const wrapper = mount(AddStationScreen);
    await wrapper.find('[data-test-id="deploy-new-station-radio"] input').setValue(true);
    await wrapper.find('[data-test-id="continue-button"]').trigger('click');
    expect(wrapper.find('[data-test-id="deploy-station-screen"]').exists()).toBe(true);

    await wrapper.find('[data-test-id="back-button"]').trigger('click');
    expect(wrapper.find('[data-test-id="split-screen"]').exists()).toBe(true);
  });
});
