import type { StarlightUserConfig } from '@astrojs/starlight/types';
import { group } from './config/sidebar';

/**
 * Starlight sidebar configuration object for the global site sidebar.
 *
 * - Top-level groups differentiate audience (Users / Dev).
 * - Use the `group()` utility function to define groups. This uses labels from our
 *   `src/content/nav/*.ts` files instead of defining labels and translations inline.
 *
 */
export const sidebar = [
  // User docs
  group('users', {
    items: [
      'users/welcome',
      'users/getting-started',
      'users/managing-assets',
      'users/wallet-management',
      'users/permissions-policies',
      'users/requests',
      'users/external-canisters',
      // 'users/upgrades',
      'users/glossary',
      // 'users/disaster-recovery',
      // 'users/limitations',
    ],
  }),

  // Developer docs
  group('developers', {
    items: [
      'developers/getting-started',
      'developers/glossary',
      'developers/i18n',
      'developers/deployment',
      'developers/contribute',
    ],
  }),
] satisfies StarlightUserConfig['sidebar'];
