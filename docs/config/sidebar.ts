import type starlight from '@astrojs/starlight';
import enLabels from '../src/content/nav/en';

type StarlightSidebarConfig = NonNullable<Parameters<typeof starlight>[0]['sidebar']>;
type StarlightSidebarEntry = StarlightSidebarConfig[number];
type StarlightManualSidebarGroup = Extract<StarlightSidebarEntry, { items: unknown }>;
type StarlightAutoSidebarGroup = Extract<
  StarlightSidebarEntry,
  { autogenerate: { directory: string; collapsed?: boolean | undefined } }
>;

type NavKey = keyof typeof enLabels;
type NavDict = Record<NavKey, string>;

const translations = Object.entries(
  import.meta.glob<{ default: NavDict }>('../src/content/nav/*.ts', { eager: true }),
)
  .map(([path, mod]) => [path.split('/').pop()!.replace('.ts', ''), mod.default] as const)
  .reduce(
    (translations, [lang, dict]) => {
      for (const _k in dict) {
        const key = _k as NavKey;
        translations[key] ??= {};
        translations[key][lang] = dict[key];
      }
      return translations;
    },
    {} as Record<NavKey, Record<string, string>>,
  );

/**
 * Create a Starlight sidebar group config entry that uses labels and translations from
 * `src/content/nav/*` files.
 */
export function group(
  key: NavKey,
  group: Omit<StarlightManualSidebarGroup, 'label'>,
): StarlightManualSidebarGroup;
export function group(
  key: NavKey,
  group: Omit<StarlightAutoSidebarGroup, 'label'>,
): StarlightAutoSidebarGroup;
/* eslint-disable  @typescript-eslint/no-explicit-any */
export function group(key: NavKey, group: any) {
  return {
    label: enLabels[key],
    translations: translations[key],
    ...group,
  };
}
