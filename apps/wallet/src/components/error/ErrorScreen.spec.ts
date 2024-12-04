import { describe, expect, it } from 'vitest';
import { mockRouter, mount } from '~/test.utils';
import ErrorScreen from './ErrorScreen.vue';
import { defaultHomeRoute } from '~/configs/routes.config';

describe('ErrorScreen', () => {
  it('renders correctly', () => {
    mockRouter.addRoute({
      name: defaultHomeRoute,
      path: '/',
      redirect: '',
      component: { template: '<div></div>' },
      children: [],
    });
    const wrapper = mount(ErrorScreen);
    expect(wrapper.exists()).toBe(true);
  });
});
