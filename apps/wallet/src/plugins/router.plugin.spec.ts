import { createPinia, setActivePinia } from 'pinia';
import { beforeEach, describe, expect, it } from 'vitest';
import { defineComponent } from 'vue';
import {
  NavigationFailure,
  NavigationFailureType,
  START_LOCATION,
  createMemoryHistory,
  createRouter,
  isNavigationFailure,
} from 'vue-router';
import { routeLoadingStateHook } from '~/plugins/router.plugin';
import { useAppStore } from '~/stores/app.store';

const createTestRouter = (opts: { abortNavigations?: boolean; slowGuards?: boolean } = {}) => {
  const router = createRouter({
    history: createMemoryHistory(),
    routes: [
      {
        path: '/:pathMatch(.*)*',
        component: defineComponent({ template: '<div></div>' }),
      },
    ],
  });

  if (opts.slowGuards) {
    router.beforeEach(async () => {
      await new Promise(resolve => setTimeout(resolve));
    });
  }

  if (opts.abortNavigations) {
    router.beforeEach(() => false);
  }

  return router;
};

const expectFailure = (
  failure: NavigationFailure | void | undefined,
  type: NavigationFailureType,
): NavigationFailure => {
  if (!isNavigationFailure(failure, type)) {
    throw new Error(`Expected a navigation failure of type ${type}`);
  }

  return failure;
};

describe('routeLoadingStateHook', () => {
  beforeEach(() => {
    setActivePinia(createPinia());
  });

  it('ends the loading state when the navigation completes', () => {
    const app = useAppStore();
    app.loading = true;

    routeLoadingStateHook(START_LOCATION, START_LOCATION, undefined);

    expect(app.loading).toBe(false);
  });

  it('ends the loading state when the navigation is aborted', async () => {
    const app = useAppStore();
    const router = createTestRouter({ abortNavigations: true });
    const failure = expectFailure(await router.push('/a'), NavigationFailureType.aborted);

    app.loading = true;
    routeLoadingStateHook(failure.to, failure.from, failure);

    expect(app.loading).toBe(false);
  });

  it('ends the loading state when the navigation is duplicated', async () => {
    const app = useAppStore();
    const router = createTestRouter();
    await router.push('/a');
    const failure = expectFailure(await router.push('/a'), NavigationFailureType.duplicated);

    app.loading = true;
    routeLoadingStateHook(failure.to, failure.from, failure);

    expect(app.loading).toBe(false);
  });

  it('keeps the loading state when the navigation was cancelled by a newer navigation', async () => {
    const app = useAppStore();
    const router = createTestRouter({ slowGuards: true });
    const [cancelled, completed] = await Promise.all([router.push('/a'), router.push('/b')]);
    const failure = expectFailure(cancelled, NavigationFailureType.cancelled);
    expect(completed).toBeUndefined();

    app.loading = true;
    routeLoadingStateHook(failure.to, failure.from, failure);

    expect(app.loading).toBe(true);
  });

  it('ends the loading state only once the superseding navigation completes', async () => {
    const app = useAppStore();
    const router = createTestRouter({ slowGuards: true });
    const loadingStates: boolean[] = [];

    router.afterEach((to, from, failure) => {
      routeLoadingStateHook(to, from, failure);
      loadingStates.push(app.loading);
    });

    app.loading = true;
    await Promise.all([router.push('/a'), router.push('/b')]);

    // the cancelled navigation keeps the loading state, the completed one ends it
    expect(loadingStates).toEqual([true, false]);
    expect(router.currentRoute.value.path).toBe('/b');
  });
});
