// @ts-check
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';
import rehypeMermaid from 'rehype-mermaid';
import { sidebar } from './astro.sidebar';

// https://astro.build/config
export default defineConfig({
  site: 'https://docs.orbit.global',
  integrations: [
    starlight({
      title: 'Orbit Documentation',
      defaultLocale: 'root',
      favicon: '/favicon.ico',
      locales: {
        root: {
          label: 'English',
          lang: 'en',
        },
      },
      logo: {
        light: './src/assets/logo.svg',
        dark: './src/assets/logo-dark.svg',
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
        Head: './src/components/Head.astro',
        Sidebar: './src/components/Sidebar.astro',
      },
    }),
  ],
  markdown: {
    rehypePlugins: [rehypeMermaid],
  },
});
