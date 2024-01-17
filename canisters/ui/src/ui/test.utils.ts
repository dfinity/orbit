import { mount as componentMount, ComponentMountingOptions } from '@vue/test-utils';
import { vi } from 'vitest';
import { Component } from 'vue';
import { i18n, navigation, pinia, serviceManager, vuetify } from '~/ui/modules';

export const mount = <T extends Component>(
  component: T,
  options: ComponentMountingOptions<T> = {},
) => {
  const mocks = options.global?.mocks || {};
  const plugins = options.global?.plugins || [];

  return componentMount(component, {
    ...options,
    global: {
      ...options.global,
      plugins: [pinia, vuetify, i18n, serviceManager, navigation, ...plugins],
      mocks: {
        $router: {
          push: vi.fn(),
        },
        ...mocks,
      },
    },
  });
};
