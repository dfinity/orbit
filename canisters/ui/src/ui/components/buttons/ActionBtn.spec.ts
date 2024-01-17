import { describe, expect, it } from 'vitest';
import { mount } from '~/ui/test.utils';
import ActionBtn from './ActionBtn.vue';

describe('ActionBtn', () => {
  it('renders properly', () => {
    const wrapper = mount(ActionBtn);

    expect(wrapper.exists()).toBe(true);
  });

  it('opens the dialog onclick', async () => {
    const wrapper = mount(ActionBtn, {
      props: {
        title: 'Hello World',
      },
      attrs: {
        'data-test-id': 'action-btn',
      },
      attachTo: document.body,
    });

    expect(wrapper.exists()).toBe(true);
    expect(document.querySelector('[data-test-id="action-btn-dialog"]')).toBeNull();

    await wrapper.find('[data-test-id="action-btn"]').trigger('click');

    // VDialog is teleported to the body element, hence the need to use document here
    const dialog = document.querySelector('[data-test-id="action-btn-dialog"]');
    expect(dialog).not.toBeNull();

    if (dialog) {
      const title = dialog.querySelector('[data-test-id="action-btn-dialog-title"]');
      expect(title?.textContent).toEqual('Hello World');
    }
  });
});
