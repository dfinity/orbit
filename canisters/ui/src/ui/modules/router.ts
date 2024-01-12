import { NavigationGuard, createRouter, createWebHistory } from 'vue-router';
import { routes, Routes } from '~/ui/config/routes';
import { initStateGuard } from '~/ui/modules';
import { useSessionStore } from '~/ui/stores/session';
import { RequiredSessionState } from '~/ui/types';
import { hasRequiredPrivilege, hasRequiredSession } from '~/ui/utils/auth';
import { i18nRouteGuard } from './i18n';
import { services } from './services';

export const redirectToKey = 'redirectTo';
export const defaultLoginRoute = Routes.Login;
export const defaultHomeRoute = Routes.Overview;

const router = createRouter({
  history: createWebHistory(services().routes.baseUrl),
  routes,
});

export const routeAccessGuard: NavigationGuard = async (to, _from, next) => {
  const session = useSessionStore();

  if (to.name === Routes.Disconnected && session.data.selectedWallet.hasAccess) {
    return next({ name: defaultHomeRoute });
  }

  if (to.name === Routes.Initialization && (!session.isAuthenticated || session.hasWallets)) {
    return next({ name: defaultHomeRoute });
  }

  if (to.name !== Routes.Initialization && session.isAuthenticated && !session.hasWallets) {
    return next({ name: Routes.Initialization });
  }

  const matchesRequiredSession = hasRequiredSession(to.meta.auth.check.session);
  if (!matchesRequiredSession) {
    let redirectToRoute = defaultHomeRoute;
    switch (to.meta.auth.check.session) {
      case RequiredSessionState.Authenticated:
        redirectToRoute = defaultLoginRoute;
        break;
      case RequiredSessionState.ConnectedToWallet: {
        redirectToRoute = Routes.Disconnected;
        break;
      }
    }

    return next({ name: redirectToRoute });
  }

  const matchesRequiredPrivilege = hasRequiredPrivilege({ anyOf: to.meta.auth.check.privileges });
  if (!matchesRequiredPrivilege) {
    return next({ name: Routes.Unauthorized });
  }

  return next();
};

router.beforeEach(initStateGuard);
router.beforeEach(i18nRouteGuard);
router.beforeEach(routeAccessGuard);

export { router };
