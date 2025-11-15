import withNuxt from './.nuxt/eslint.config.mjs';
import stylistic from '@stylistic/eslint-plugin';
import prettier from 'eslint-plugin-prettier/recommended';

export default withNuxt(
  stylistic.configs.recommended,
  {
    ignores: ['public/*'],
    plugins: {
      '@stylistic': stylistic,
    },
    rules: {
      'no-undef': 'off',
      'no-useless-catch': 'off',
      'vue/no-v-html': 'off',
      'vue/prop-name-casing': 'off',
      'vue/multi-word-component-names': 'off',
      '@stylistic/indent': ['error', 2],
    },
  },
  prettier,
);
