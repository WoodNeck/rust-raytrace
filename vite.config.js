import { defineConfig } from 'vite'
import mkcert from 'vite-plugin-mkcert'
import crossOriginIsolation from 'vite-plugin-cross-origin-isolation'

export default defineConfig({
  server: { https: true },
  plugins: [
    mkcert(),
    crossOriginIsolation()
  ]
})
