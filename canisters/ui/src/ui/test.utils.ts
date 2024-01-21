import { mount as componentMount, ComponentMountingOptions } from '@vue/test-utils';
import { vi } from 'vitest';
import { Component, createApp } from 'vue';
import { i18n, navigation, serviceManager, vuetify } from '~/ui/modules';
import { createTestingPinia } from '@pinia/testing';

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
      plugins: [createTestingPinia(), vuetify, i18n, serviceManager, navigation, ...plugins],
      mocks: {
        $router: {
          push: vi.fn(),
        },
        ...mocks,
      },
    },
  });
};

export function loadComposable<T>(loader: () => T): [T, ReturnType<typeof createApp>] {
  let result: T;

  const app = createApp({
    setup() {
      result = loader();
      return () => {};
    },
  });

  app.mount(document.createElement('div'));

  return [result!, app];
}
