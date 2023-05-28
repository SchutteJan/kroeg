import { defineConfig } from 'npm:vite@^4.0.4'
import { svelte } from 'npm:@sveltejs/vite-plugin-svelte@^2.0.2'

import autoprefixer from "https://jspm.dev/autoprefixer"
import postcssNesting from 'npm:postcss-nesting@11.2.2'

import 'npm:svelte@^3.54.0'
import * as _router from 'npm:svelte-routing@1.8.9'


// https://vitejs.dev/config/
export default defineConfig({
  plugins: [svelte()],
  server: {
    cors: false,
  },
  css: {
    postcss: {
      plugins: [
        autoprefixer, postcssNesting
      ]
    }
  }
})
