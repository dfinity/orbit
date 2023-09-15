import {
  mdiBankTransfer,
  mdiBookOpenVariant,
  mdiCogs,
  mdiHome,
  mdiLogoutVariant,
  mdiWalletBifold,
} from '@mdi/js';
import { App } from 'vue';
import { useAuthStore } from '~/ui/stores';
import { NavigationActionType, NavigationSection } from '~/ui/types';

const mainNavigation: NavigationSection[] = [
  {
    name: 'main',
    localeKey: 'navigation.main.name',
    items: [
      {
        name: 'home',
        localeKey: 'navigation.main.items.home',
        action: {
          type: NavigationActionType.To,
          handle: route => (route.params.locale ? `/${route.params.locale}/home` : '/home'),
        },
        icon: mdiHome,
      },
      {
        name: 'wallets',
        localeKey: 'navigation.main.items.wallets',
        action: {
          type: NavigationActionType.To,
          handle: route => (route.params.locale ? `/${route.params.locale}/wallets` : '/wallets'),
        },
        icon: mdiWalletBifold,
      },
      {
        name: 'transactions',
        localeKey: 'navigation.main.items.transactions',
        action: {
          type: NavigationActionType.To,
          handle: route =>
            route.params.locale ? `/${route.params.locale}/transactions` : '/transactions',
        },
        icon: mdiBankTransfer,
      },
      {
        name: 'address-book',
        localeKey: 'navigation.main.items.address_book',
        action: {
          type: NavigationActionType.To,
          handle: route =>
            route.params.locale ? `/${route.params.locale}/address-book` : '/address-book',
        },
        icon: mdiBookOpenVariant,
      },
    ],
  },
  {
    name: 'configuration',
    localeKey: 'navigation.configuration.name',
    items: [
      {
        name: 'settings',
        localeKey: 'navigation.configuration.items.settings',
        action: {
          type: NavigationActionType.To,
          handle: route => (route.params.locale ? `/${route.params.locale}/settings` : '/settings'),
        },
        icon: mdiCogs,
      },
      {
        name: 'logout',
        localeKey: 'navigation.configuration.items.logout',
        action: {
          type: NavigationActionType.Callback,
          handle: async () => {
            const auth = useAuthStore();

            await auth.signOut();
          },
        },
        icon: mdiLogoutVariant,
      },
    ],
  },
];

class Navigation {
  constructor(
    private readonly navigation: {
      main: NavigationSection[];
    },
  ) {}

  install(app: App): void {
    app.config.globalProperties.$navigation = this.navigation;
  }
}

declare module 'vue' {
  interface ComponentCustomProperties {
    $navigation: {
      main: NavigationSection[];
    };
  }
}

export const navigation = new Navigation({
  main: mainNavigation,
});
