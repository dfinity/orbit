const baseConfig = require("../../.eslintrc.cjs");

/** @type {import('eslint').Linter.Config} */
const config = {
  ...baseConfig,
  root: false,
  extends: [
    ...baseConfig.extends,
    'plugin:vue/vue3-recommended',
    'prettier',
  ],
  parser: "vue-eslint-parser",
  parserOptions: {
    parser: "@typescript-eslint/parser",
    ecmaVersion: 2020,
    sourceType: "module"
  }
};

module.exports = config;
