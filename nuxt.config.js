// https://nuxt.com/docs/api/configuration/nuxt-config

const vitePlugins = [];
const isDev = process.env.NODE_ENV === 'development';

export default defineNuxtConfig({
  compatibilityDate: '2025-10-29',
  components: [
    '@/components',
    { path: '@/components/common', prefix: 'C' },
    { path: '@/components/animation', prefix: 'A' },
    { path: '@/components/ui', prefix: 'Ui' },
    { path: '@/components/layout', prefix: 'L' },
  ],
  css: ['reset-css', '@/assets/styles/base/index.scss'],
  devServer: {
    host: '0',
  },
  devtools: { enabled: isDev },
  features: { inlineStyles: false },
  ignore: ['**/src-tauri/**'],
  modules: ['@pinia/nuxt', '@vueuse/nuxt', 'nuxt-swiper', 'nuxt-svgo'],
  ssr: false,
  svgo: { defaultImport: 'component', explicitImportsOnly: true },
  vite: {
    clearScreen: false,
    envPrefix: ['VITE_', 'TAURI_'],
    server: {
      strictPort: true,
    },
    css: {
      preprocessorOptions: {
        scss: {
          additionalData: '@use "@/assets/styles/utils" as *;',
          silenceDeprecations: ['global-builtin', 'import'],
        },
      },
    },
    plugins: vitePlugins,
  },
});
