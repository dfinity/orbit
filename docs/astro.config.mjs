// @ts-check
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';
import rehypeMermaid from 'rehype-mermaid';
import { sidebar } from './astro.sidebar';

// https://astro.build/config
export default defineConfig({
  site: 'https://docs.orbitwallet.io',
  integrations: [
    starlight({
      title: 'Orbit Documentation',
      defaultLocale: 'root',
      locales: {
        root: {
          label: 'English',
          lang: 'en',
        },
      },
      logo: {
        dark: './src/assets/orbit-logo-light.svg',
        light: './src/assets/orbit-logo-dark.svg',
        replacesTitle: true,
      },
      social: {
        github: 'https://github.com/dfinity/orbit',
      },
      editLink: {
        baseUrl: 'https://github.com/dfinity/orbit/edit/main/docs/',
      },
      customCss: ['./src/fonts/font-face.css', './src/styles/global.css'],
      sidebar,
      components: {
        // See https://docs.astro.build/reference/components
        Sidebar: './src/components/starlight/Sidebar.astro',
      },
    }),
  ],
  markdown: {
    rehypePlugins: [rehypeMermaid],
  },
});
