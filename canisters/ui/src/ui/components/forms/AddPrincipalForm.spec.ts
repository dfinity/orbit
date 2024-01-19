import { describe, expect, it } from 'vitest';
import { mount } from '~/ui/test.utils';
import AddPrincipalForm from './AddPrincipalForm.vue';

describe('AddPrincipalForm', () => {
  it('renders properly', () => {
    const wrapper = mount(AddPrincipalForm, {
      props: {
        modelValue: 'Test',
      },
    });

    expect(wrapper.exists()).toBe(true);
  });

  it('renders the prepend slot', () => {
    const wrapper = mount(AddPrincipalForm, {
      props: {
        modelValue: 'Test',
      },
      slots: {
        prepend: '<div class="added"></div>',
      },
    });

    expect(wrapper.exists()).toBe(true);
    expect(wrapper.find('.added').exists()).toBe(true);
  });

  it('changes the model on changes', async () => {
    const wrapper = mount(AddPrincipalForm, {
      props: {
        modelValue: 'Test',
      },
    });

    expect(wrapper.exists()).toBe(true);

    const input = wrapper.find('input');
    expect(input.exists()).toBe(true);

    await input.setValue('55wqu-ejnwv-qcv3k-77ing-fdsxt-rtr6j-ryfct-b7xgx-fjzqm-hb667-jae');

    expect(wrapper.emitted('update:modelValue')).toEqual([
      ['55wqu-ejnwv-qcv3k-77ing-fdsxt-rtr6j-ryfct-b7xgx-fjzqm-hb667-jae'],
    ]);
  });

  it('submits the form and emits it', async () => {
    const wrapper = mount(AddPrincipalForm, {
      props: {
        modelValue: '55wqu-ejnwv-qcv3k-77ing-fdsxt-rtr6j-ryfct-b7xgx-fjzqm-hb667-jae',
      },
    });

    const form = wrapper.findComponent({ ref: 'form' });
    await form.trigger('submit');

    await wrapper.vm.$nextTick();

    expect(wrapper.emitted('submit')).toEqual([
      ['55wqu-ejnwv-qcv3k-77ing-fdsxt-rtr6j-ryfct-b7xgx-fjzqm-hb667-jae'],
    ]);
  });
});
