import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import { VitePWA } from 'vite-plugin-pwa'

export default defineConfig({
  plugins: [
    react(),
    VitePWA({
      registerType: 'autoUpdate',
      manifest: {
        name: 'Cortex',
        short_name: 'Cortex',
        description: 'Base de conhecimento pessoal com RAG',
        theme_color: '#0a0a0a',
        background_color: '#0a0a0a',
        display: 'standalone',
        start_url: '/',
      },
    }),
  ],
  server: {
    proxy: {
      '/health': 'http://127.0.0.1:8080',
    },
  },
})
