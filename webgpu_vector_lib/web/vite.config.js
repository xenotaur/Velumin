import { defineConfig } from 'vite'
import path from 'path'

export default defineConfig({
  server: {
    fs: {
      allow: [
        // Allow serving files from the parent directory
        path.resolve(__dirname, '../pkg')
      ]
    }
  },
  resolve: {
    alias: {
      // Alias the `pkg/` folder (one level up)
      '@pkg': path.resolve(__dirname, '../pkg')
    }
  }
})
