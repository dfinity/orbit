import { describe, expect, it } from 'vitest';
import { mount } from '~/ui/test.utils';
import PageBody from './PageBody.vue';

describe('PageBody', () => {
  it('renders properly', () => {
    const wrapper = mount(PageBody);

    expect(wrapper.exists()).toBe(true);
  });

  it('renders the default slot', () => {
    const wrapper = mount(PageBody, {
      slots: {
        default: 'Hello World',
      },
    });

    expect(wrapper.exists()).toBe(true);
    expect(wrapper.find('[data-testid="page-body"]').exists()).toBe(true);
    expect(wrapper.find('[data-testid="page-body"]').text()).toBe('Hello World');
  });
});
