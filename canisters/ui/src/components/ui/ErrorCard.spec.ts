import { describe } from 'node:test';
import ErrorCard from './ErrorCard.vue';
import { mount } from '~/test.utils';
import { expect, it } from 'vitest';

describe('ErrorCard', () => {
  it('shows the error message', () => {
    const wrapper = mount(ErrorCard, {
      props: {
        error: 'An error occurred',
      },
    });

    expect(wrapper.text()).toContain('An error occurred');
  });

  it('shows the error message with a title', () => {
    const wrapper = mount(ErrorCard, {
      props: {
        title: 'Title',
        error: 'An error occurred',
      },
    });

    expect(wrapper.text()).toContain('Title');
    expect(wrapper.text()).toContain('An error occurred');
  });

  it('shows the error details', () => {
    const wrapper = mount(ErrorCard, {
      props: {
        error: 'An error occurred',
        errorDetails: 'Some details',
      },
    });

    expect(wrapper.find('[data-test-id="error-details-panel"]').exists()).toBeTruthy();
  });

  it('does not show the details by default', () => {
    const wrapper = mount(ErrorCard, {
      props: {
        error: 'An error occurred',
      },
    });

    expect(wrapper.find('[data-test-id="error-details-panel"]').exists()).toBeFalsy();
  });
});
