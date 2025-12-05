import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import tsconfigPaths from 'vite-tsconfig-paths'

import tailwindcss from '@tailwindcss/vite'
// https://vite.dev/config/
export default defineConfig({
  plugins: [tsconfigPaths(), react(), tailwindcss()],
  // resolve: {
  //   alias: {
  //     '@alex-wine/napi-test-wasm32-wasi': path.resolve(__dirname, '../packages/bindings/napi-test.wasi-browser.js'),
  //   }
  // }
})
