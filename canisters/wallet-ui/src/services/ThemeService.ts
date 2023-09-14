import { SupportedTheme } from '~/types';

export class ThemeService {
  static readonly storageKey = 'theme';

  resolveTheme(): SupportedTheme {
    const storedTheme = localStorage.getItem(ThemeService.storageKey);
    if (storedTheme && Object.values(SupportedTheme).includes(storedTheme as SupportedTheme)) {
      const selectedTheme = storedTheme as SupportedTheme;

      return selectedTheme;
    }

    const theme =
      window.matchMedia &&
      window.matchMedia(`(prefers-color-scheme: ${SupportedTheme.Dark})`).matches
        ? SupportedTheme.Dark
        : SupportedTheme.Light;

    return theme;
  }

  updateUserTheme(theme: SupportedTheme): void {
    localStorage.setItem(ThemeService.storageKey, theme);
  }
}
