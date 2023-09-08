import inject from '@rollup/plugin-inject';
import vue from '@vitejs/plugin-vue';
import { defineConfig, UserConfig } from 'vite';
import { resolve } from 'path';

// https://vitejs.dev/config/
export default defineConfig(({ mode }): UserConfig => {
  const isProduction = mode === 'production';

  return {
    base: '/',
    root: '.',
    publicDir: './public',
    appType: 'spa',
    // Vite automatically loads .env files from the root of the project
    // if they are prefixed with the envPrefix.
    envPrefix: 'APP_',
    plugins: [vue()],
    build: {
      target: 'es2020',
      sourcemap: !isProduction,
      minify: isProduction,
      chunkSizeWarningLimit: 500,
      outDir: './dist',
      emptyOutDir: true,
      rollupOptions: {
        input: {
          main: './index.html',
        },
        plugins: [
          inject({
            modules: {
              // Polyfill Buffer for production build
              Buffer: ['buffer', 'Buffer'],
            },
          }),
        ],
      },
    },
    css: {
      devSourcemap: !isProduction,
    },
    define: {
      // Vite env variable replacements for the runtime.
      //
      // Make sure to use import.meta.env as the prefix since
      // vite uses that during runtime to access the variables.
      // https://vitejs.dev/guide/env-and-mode.html#env-variables
      'import.meta.env.APP_VERSION': JSON.stringify(`v${process.env.npm_package_version}`),
    },
    resolve: {
      alias: {
        '~': resolve('src'),
      },
    },
  };
});
