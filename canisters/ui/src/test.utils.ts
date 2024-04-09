/* eslint-disable vue/one-component-per-file */
import { createTestingPinia } from '@pinia/testing';
import { mount as componentMount, ComponentMountingOptions } from '@vue/test-utils';
import { StateTree } from 'pinia';
import { Component, ComponentPublicInstance, createApp, defineComponent, Ref } from 'vue';
import { createMemoryHistory, createRouter } from 'vue-router';
import { i18n } from '~/plugins/i18n.plugin';
import { navigation } from '~/plugins/navigation.plugin';
import { serviceManager } from '~/plugins/services.plugin';
import { vuetify } from '~/plugins/vuetify.plugin';

export const mockRouter = createRouter({
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
        vuetify(),
        i18n,
        serviceManager,
        navigation.withSections({ main: [] }),
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
  app.use(vuetify());
  app.use(i18n);
  app.use(serviceManager);
  app.use(navigation.withSections({ main: [] }));

  const container = document.createElement('div');

  return app.mount(container) as ComponentPublicInstance<Props, RawBindings>;
};

export function createMockRef<T>(value: T): Ref<T> {
  return { value } as Ref<T>;
}

export function mockFileReader(result: ArrayBuffer, errorOut: boolean = false): typeof FileReader {
  return class MockFileReader {
    result?: ArrayBuffer = result;
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    onload: any = null;
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    onerror: any | null = null;

    readAsArrayBuffer(_file: File): void {
      this.result = result;

      if (errorOut) {
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        setTimeout(() => this.onerror?.(new ProgressEvent('error')));
      } else {
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        setTimeout(() => this.onload?.(new ProgressEvent('load', { target: this } as any)));
      }
    }

    // Mimicking static properties of the FileReader
    static EMPTY = 0;
    static LOADING = 1;
    static DONE = 2;
  } as unknown as typeof FileReader;
}
