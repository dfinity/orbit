import { describe, expect, it } from 'vitest';
import { mount } from '~/test.utils';
import ChangeCanisterActionBtn from './ChangeCanisterActionBtn.vue';

describe('ChangeCanisterActionBtn', () => {
  it('renders action btn', () => {
    const wrapper = mount(ChangeCanisterActionBtn);

    expect(wrapper.exists()).toBe(true);
  });

  it('on click emits editing', async () => {
    const wrapper = mount(ChangeCanisterActionBtn);
    const btn = wrapper.find('[data-test-id="submit-upgrade-btn"]');

    expect(btn.exists()).toBe(true);

    await btn.trigger('click');

    console.log(wrapper.emitted());

    expect(wrapper.emitted('editing')).toBeTruthy();
  });
});
