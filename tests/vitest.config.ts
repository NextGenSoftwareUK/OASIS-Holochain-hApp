import { defineConfig } from 'vitest/config'

export default defineConfig({
  test: {
    include: ['src/oasis/oasis/contract.test.ts'],
    fileParallelism: false,
    testTimeout: 60*1000*3 // 3  mins
  },
})

