import { inject, provide } from 'vue';

const VERIFIED_READS_KEY = 'verifiedReads';

/**
 * Marks the current component subtree as part of the request review dialog.
 *
 * Shared components rendered inside it (autocompletes, named rule labels, etc.) then load their data
 * with the verified actor, the same way the request itself is loaded.
 */
export const provideVerifiedReads = (): void => {
  provide(VERIFIED_READS_KEY, true);
};

/**
 * Returns true inside a subtree marked with `provideVerifiedReads`, meant to be passed as the
 * `verifiedCall` argument of station service reads. Must be called from a component setup.
 */
export const useVerifiedReads = (): boolean => inject<boolean>(VERIFIED_READS_KEY, false);
