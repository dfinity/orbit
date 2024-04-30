import { describe } from 'node:test';
import TextLabel from './TextLabel.vue';
import { mount } from '~/test.utils';
import { expect, it } from 'vitest';

describe('TextLabel', () => {
  it('renders the text', () => {
    const wrapper = mount(TextLabel, {
      props: {
        label: 'Hello, world!',
      },
    });

    expect(wrapper.text()).toContain('Hello, world!');
  });

  it('renders the text with a tooltip', () => {
    const wrapper = mount(TextLabel, {
      props: {
        label: 'Hello, world!',
        tooltip: 'This is a tooltip',
      },
    });

    expect(wrapper.text()).toContain('Hello, world!');
    expect(wrapper.find('[data-test-id="open-tooltip-button"]').exists()).toBeTruthy();
  });
});
