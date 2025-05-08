import { describe, it, expect } from 'vitest';
import { mount } from '~/test.utils';
import DiffView from './DiffView.vue';

describe('DiffView', () => {
  it('does not show diff when values are equal', () => {
    const wrapper = mount(DiffView, {
      props: {
        beforeValue: 'same',
        afterValue: 'same',
      },
      slots: {
        default: ({ value, diffMode, showDiff }) => `${value} - ${diffMode} - ${showDiff}`,
      },
    });

    const node = wrapper.find('[data-diff-mode="after"]');
    expect(node.text()).toBe('same - after - false');
  });

  it('shows diff when values differ', () => {
    const wrapper = mount(DiffView, {
      props: {
        beforeValue: 'A',
        afterValue: 'B',
      },
      slots: {
        default: ({ value, diffMode, showDiff }) => `${value} - ${diffMode} - ${showDiff}`,
      },
    });
    const beforeNode = wrapper.find('[data-diff-mode="before"]');
    const afterNode = wrapper.find('[data-diff-mode="after"]');
    expect(beforeNode.text()).toBe('A - before - true');
    expect(afterNode.text()).toBe('B - after - true');
  });

  it('honors compareValues prop to suppress diff', () => {
    const compare = () => true; // always equal
    const wrapper = mount(DiffView, {
      props: {
        beforeValue: 'foo',
        afterValue: 'bar',
        compareValues: compare,
      },
      slots: {
        default: ({ value, diffMode, showDiff }) => `${value} - ${diffMode} - ${showDiff}`,
      },
    });
    expect(wrapper.findAll('[data-diff-mode]')).toHaveLength(1);
    const node = wrapper.find('[data-diff-mode]');
    expect(node.text()).toBe('bar - after - false');
  });
});
