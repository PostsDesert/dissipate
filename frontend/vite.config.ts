import { defineConfig } from 'vite'
import solidPlugin from 'vite-plugin-solid'
import { VitePWA } from 'vite-plugin-pwa'

export default defineConfig({
  plugins: [
    solidPlugin(),
    VitePWA({
      registerType: 'autoUpdate',
      manifest: './pwa-manifest.json',
      workbox: {
        globPatterns: ['**/*.{js,css,html,ico,png,svg}'],
        runtimeCaching: [
          {
            urlPattern: /^https:\/\/api\.dissipate\.com\/.*/i,
            handler: 'NetworkFirst',
            options: {
              cacheName: 'api-cache',
              expiration: {
                maxEntries: 100,
                maxAgeSeconds: 86400
              }
            }
          }
        ]
      }
    })
  ],
  server: {
    port: 5173
  }
})
