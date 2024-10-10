import { describe, expect, it } from 'vitest';
import { mount } from '~/test.utils';
import JsonVizualizer from './JsonVizualizer.vue';

describe('JsonVizualizer', () => {
  it('mount the component', () => {
    const wrapper = mount(JsonVizualizer, {
      props: {
        data: { foo: 'bar' },
      },
    });

    const json = wrapper.find('pre').text();

    expect(json).toContain(JSON.stringify({ foo: 'bar' }, null, 2));
  });

  it('mount the component with interactive options', () => {
    const wrapper = mount(JsonVizualizer, {
      props: {
        data: { foo: 'bar' },
        interactive: true,
      },
    });

    expect(wrapper.find('[data-test-id="toggle-remove-undefined-or-null"]').exists()).toBe(true);
  });

  it('mount the component without interactive options', () => {
    const wrapper = mount(JsonVizualizer, {
      props: {
        data: { foo: 'bar' },
        interactive: false,
      },
    });

    expect(wrapper.find('[data-test-id="toggle-remove-undefined-or-null"]').exists()).toBe(false);
  });

  it('shows btn to show more data', () => {
    const wrapper = mount(JsonVizualizer, {
      props: {
        data: {
          line1: 'line1',
          line2: 'line2',
          line3: 'line3',
        },
        rows: 2,
      },
    });

    expect(wrapper.find('[data-test-id="show-more-btn"]').exists()).toBe(true);
    expect(wrapper.find('[data-test-id="show-less-btn"]').exists()).toBe(false);
  });

  it('shows btn to show less data', async () => {
    const wrapper = mount(JsonVizualizer, {
      props: {
        data: {
          line1: 'line1',
          line2: 'line2',
          line3: 'line3',
        },
        rows: 2,
      },
    });

    await wrapper.find('[data-test-id="show-more-btn"]').trigger('click');

    expect(wrapper.find('[data-test-id="show-more-btn"]').exists()).toBe(false);
    expect(wrapper.find('[data-test-id="show-less-btn"]').exists()).toBe(true);
  });
});
