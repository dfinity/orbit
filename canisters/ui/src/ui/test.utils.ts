/* eslint-disable vue/one-component-per-file */
import { createTestingPinia } from '@pinia/testing';
import { mount as componentMount, ComponentMountingOptions } from '@vue/test-utils';
import { StateTree } from 'pinia';
import { Component, ComponentPublicInstance, createApp, defineComponent, Ref } from 'vue';
import { createMemoryHistory, createRouter } from 'vue-router';
import { i18n, navigation, serviceManager, vuetify } from '~/ui/modules';

const mockRouter = createRouter({
  history: createMemoryHistory(),
  routes: [
    {
      path: '/:pathMatch(.*)*',
      component: defineComponent({
        template: '<div><slot></slot></div>',
      }),
    },
  ],
});

export const mount = <T extends Component>(
  component: T,
  options: ComponentMountingOptions<T> = {},
  { initialPiniaState }: { initialPiniaState?: StateTree } = {},
) => {
  const mocks = options.global?.mocks || {};
  const plugins = options.global?.plugins || [];

  return componentMount(component, {
    ...options,
    global: {
      ...options.global,
      plugins: [
        createTestingPinia({
          initialState: initialPiniaState,
        }),
        vuetify,
        i18n,
        serviceManager,
        navigation,
        mockRouter,
        ...plugins,
      ],
      mocks: {
        ...mocks,
      },
    },
  });
};

export const setupComponent = <Props, RawBindings>(
  setupFunction: () => RawBindings,
): ComponentPublicInstance<Props, RawBindings> => {
  const app = createApp({
    setup: setupFunction,
    template: '<div></div>',
  });

  app.use(createTestingPinia());
  app.use(mockRouter);
  app.use(vuetify);
  app.use(i18n);
  app.use(serviceManager);
  app.use(navigation);

  const container = document.createElement('div');

  return app.mount(container) as ComponentPublicInstance<Props, RawBindings>;
};

export function createMockRef<T>(value: T): Ref<T> {
  return { value } as Ref<T>;
}
