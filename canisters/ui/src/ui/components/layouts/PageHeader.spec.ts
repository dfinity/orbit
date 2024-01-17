import { describe, expect, it } from 'vitest';
import { mount } from '~/ui/test.utils';
import PageHeader from './PageHeader.vue';

describe('PageHeader', () => {
  it('renders properly', () => {
    const wrapper = mount(PageHeader);

    expect(wrapper.exists()).toBe(true);
  });

  it('renders the title slot with the title prop', () => {
    const wrapper = mount(PageHeader, {
      props: {
        title: 'Hello World',
      },
    });

    expect(wrapper.exists()).toBe(true);
    expect(wrapper.find('[data-testid="page-header-title"]').exists()).toBe(true);
    expect(wrapper.find('[data-testid="page-header-title"]').text()).toEqual('Hello World');
  });

  it('renders the title slot with slot replacement', () => {
    const wrapper = mount(PageHeader, {
      slots: {
        title: 'Hello World',
      },
    });

    expect(wrapper.exists()).toBe(true);
    expect(wrapper.find('[data-testid="page-header-title"]').exists()).toBe(true);
    expect(wrapper.find('[data-testid="page-header-title"]').text()).toEqual('Hello World');
  });

  it('renders the actions slot with slot replacement', () => {
    const wrapper = mount(PageHeader, {
      slots: {
        actions: 'Hello World',
      },
    });

    expect(wrapper.exists()).toBe(true);
    expect(wrapper.find('[data-testid="page-header-actions"]').exists()).toBe(true);
    expect(wrapper.find('[data-testid="page-header-actions"]').text()).toEqual('Hello World');
  });
});
