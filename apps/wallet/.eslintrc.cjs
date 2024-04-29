const baseConfig = require('../../.eslintrc.cjs');

/** @type {import('eslint').Linter.Config} */
const config = {
  ...baseConfig,
  root: false,
  env: {
    browser: true,
    es2020: true,
  },
  extends: [...baseConfig.extends, 'plugin:vue/vue3-recommended', 'prettier'],
  parser: 'vue-eslint-parser',
  parserOptions: {
    parser: '@typescript-eslint/parser',
    ecmaVersion: 2020,
    sourceType: 'module',
  },
  rules: {
    ...baseConfig.rules,
    'vue/singleline-html-element-content-newline': 0,
    'vue/component-name-in-template-casing': ['error', 'PascalCase'],
    'vue/valid-v-slot': ['error', { allowModifiers: true }],
  },
};

module.exports = config;
